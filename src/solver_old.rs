
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

use cassowary::{Constraint, Solver, Variable};
use cassowary::strength::{WEAK, MEDIUM, STRONG, REQUIRED};
use cassowary::WeightedRelation::{self, LE, EQ, GE};
use specs::{Component, Entity, Entities, Fetch, FetchMut, Join, ReadStorage, System, WriteStorage};

use ::class::*;
use ::track::BitSetJoin;

/*
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum KeyId {
    Entity(Entity),
    Context,
    Unkeyed,
}

impl Default for KeyId {
    fn default() -> KeyId {
        KeyId::Unkeyed
    }
}

/// Key for picking out variables used in the solver.
///
/// Mainly just useful or attaching some meaning to them.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Key(pub KeyId, pub &'static str);

#[derive(Clone, Default, Debug)]
pub struct Changes {
    changes: Vec<(Key, Variable, f64)>,
}

impl Changes {
    pub fn set_changes(&mut self, changes: Vec<(Key, Variable, f64)>) {
        self.changes = changes;
    }

    pub fn changes(&self) -> &Vec<(Key, Variable, f64)> {
        &self.changes
    }
}

pub struct Keys {
    key_map: HashMap<Key, Variable>,
    var_map: HashMap<Variable, Key>,
}
*/

/// Just a macro to help with the abundant boilerplate related to getting a lot of
/// components that are related to the UI and converting them into the `FlaggedStorage`s
/// so we can just iterate over the flagged portions.
macro_rules! class {
    (
        res [ $( $res_name:ident => $resource:ident, )* ]
        comp [ $( $name:ident => $component:ident, )* ]
    ) => {
        #[derive(SystemData)]
        pub struct ClassData<'a> {
            $(
                $res_name: Fetch<'a, $resource>,
            )*
            $(
                $name: ReadStorage<'a, $component>,
            )*
        }

        #[derive(SystemData)]
        pub struct ClassDataMut<'a> {
            $(
                $res_name: FetchMut<'a, $resource>,
            )*
            $(
                $name: WriteStorage<'a, $component>,
            )*
        }

        pub struct FlaggedClass<'a> {
            $(
                $name: &'a <$component as Component>::Storage,
            )*
        }

        pub struct FlaggedClassMut<'a> {
            $(
                $name: &'a mut <$component as Component>::Storage,
            )*
        }

        impl<'a, 'b> From<&'b ClassData<'a>> for FlaggedClass<'b> {
            fn from(class: &'b ClassData<'a>) -> Self {
                FlaggedClass {
                    $(
                        $name: class.$name.open().1,
                    )*
                }
            }
        }

        impl<'a, 'b> From<&'b mut ClassDataMut<'a>> for FlaggedClassMut<'b> {
            fn from(class: &'b mut ClassDataMut<'a>) -> Self {
                FlaggedClassMut {
                    $(
                        $name: (&mut class.$name).open().1,
                    )*
                }
            }
        }

        impl<'a, 'b> From<&'b ClassDataMut<'a>> for FlaggedClass<'b> {
            fn from(class: &'b ClassDataMut<'a>) -> Self {
                FlaggedClass {
                    $(
                        $name: (&class.$name).open().1,
                    )*
                }
            }
        }

        pub struct ResetSystem;
        impl<'a> System<'a> for ResetSystem {
            type SystemData = ClassDataMut<'a>;
            fn run(&mut self, mut class: Self::SystemData) {
                let mut flagged = FlaggedClassMut::from(&mut class);
                $(
                    flagged.$name.clear_flags();
                )*
            }
        }
    }
}

class!(
    res [
        viewport => Viewport,
        changes => Changes,
    ]
    comp [
        parents => Parent,
        children => Children,
        positions => Position,
        bounds => Bounds,
    ]
);

#[derive(Default)]
pub struct SolverSystem;
impl<'a> System<'a> for SolverSystem {
    type SystemData = (
        Entities<'a>,
        ClassDataMut<'a>,
    );
    fn run(&mut self, (entities, mut class): Self::SystemData) {
        // Check if the viewport was changed
        if self.dimensions[0] != class.viewport.width || self.dimensions[1] != class.viewport.height {
            self.suggest_viewport(class.viewport.width, class.viewport.height);
        }
        
        {
            let flagged = FlaggedClass::from(&class);

            for (entity, position) in (&*entities, flagged.positions).join() {
                match position.kind {
                    PositionKind::Absolute |
                    PositionKind::Relative => {
                        let left_bound_key = Key(KeyId::Entity(entity), "left_bound");
                        let upper_bound_key = Key(KeyId::Entity(entity), "upper_bound");
                        let left_align_key = Key(KeyId::Entity(entity), "left_align");
                        let top_align_key = Key(KeyId::Entity(entity), "top_align");
                        
                        let left_bound = self.fill_variable(&left_bound_key, None);
                        let upper_bound = self.fill_variable(&upper_bound_key, None);

                        self.replace_constraint(&left_align_key, left_bound |EQ(WEAK)| 0.0);
                        self.replace_constraint(&top_align_key, upper_bound |EQ(WEAK)| 0.0);

                    },
                    PositionKind::Free => { },
                }
            }

            // Set left/right/upper/lower bounds for the UI based on the parent's bounds.
            for (entity, parent, position) in (&*entities, flagged.parents, &class.positions).join() {
                match position.kind {
                    PositionKind::Absolute |
                    PositionKind::Relative => {
                        let mut bound = |string: &'static str, relation: WeightedRelation | {
                            let key = Key(KeyId::Entity(entity), string);
                            let parent_key = Key(KeyId::Entity(parent.entity), string);
                            let var = self.fill_variable(&key, None);
                            let parent_var = self.fill_variable(&parent_key, None);

                            let constraint = var |relation| parent_var;
                            self.replace_constraint(&key, constraint);
                        };

                        bound("left_bound", GE(REQUIRED));
                        bound("right_bound", LE(REQUIRED));
                        bound("upper_bound", GE(REQUIRED));
                        bound("lower_bound", LE(REQUIRED));
                    },
                    PositionKind::Free => { }, // Do nothing.
                }
            }

            // Set left/right/upper/lower bounds for the UI, falls back to the viewport for no parent.
            for (entity, _, position) in (&*entities, &BitSetJoin(flagged.parents.removed()), &class.positions).join() {
                match position.kind {
                    PositionKind::Absolute |
                    PositionKind::Relative => {
                        let width = self.fill_variable(&Key(KeyId::Context, "viewport width"), Some(REQUIRED - 1.0));
                        let height = self.fill_variable(&Key(KeyId::Context, "viewport height"), Some(REQUIRED - 1.0));
                        
                        let key = Key(KeyId::Entity(entity), "left_bound");
                        let var = self.fill_variable(&key, None);
                        let constraint = var |GE(REQUIRED)| 0.0;
                        self.replace_constraint(&key, constraint);

                        let key = Key(KeyId::Entity(entity), "right_bound");
                        let var = self.fill_variable(&key, None);
                        let constraint = var |LE(REQUIRED)| width;
                        self.replace_constraint(&key, constraint);

                        let key = Key(KeyId::Entity(entity), "upper_bound");
                        let var = self.fill_variable(&key, None);
                        let constraint = var |GE(REQUIRED)| 0.0;
                        self.replace_constraint(&key, constraint);

                        let key = Key(KeyId::Entity(entity), "lower_bound");
                        let var = self.fill_variable(&key, None);
                        let constraint = var |LE(REQUIRED)| height;
                        self.replace_constraint(&key, constraint);
                    },
                    PositionKind::Free => { }, // No bounds for free floating windows.
                }
            }

            // TODO: Set up variables and constraints for positioning UI rectangles properly
            for (entity, position) in (&*entities, flagged.positions).join() {

            }

            for (entity, bound) in (&*entities, flagged.bounds).join() {
                let left_bound_key = Key(KeyId::Entity(entity), "left_bound");
                let right_bound_key = Key(KeyId::Entity(entity), "right_bound");
                let width_key = Key(KeyId::Entity(entity), "width");

                let left_bound = self.fill_variable(&left_bound_key, None);
                let right_bound = self.fill_variable(&right_bound_key, None);
                let constraint = match bound.width.clone().unwrap_or(Coordinate::Pixel(100.0)) {
                    Coordinate::Pixel(units) => {
                        let width = self.fill_variable(&width_key, Some(MEDIUM));
                        self.solver.suggest_value(width, units);

                        let width_constraint = right_bound - left_bound |EQ(WEAK)| width;
                        width_constraint
                    },
                    Coordinate::Percent(percent) => {
                        match class.parents.get(entity) {
                            Some(parent) => {
                                let parent_left_bound = self.fill_variable(&Key(KeyId::Entity(parent.entity), "left_bound"), None);
                                let parent_right_bound = self.fill_variable(&Key(KeyId::Entity(parent.entity), "right_bound"), None);
                                right_bound - left_bound |EQ(WEAK)| (parent_right_bound - parent_left_bound) * percent
                            },
                            None => {
                                let width = self.fill_variable(&Key(KeyId::Context, "viewport width"), Some(REQUIRED - 1.0));
                                right_bound - left_bound |EQ(WEAK)| width * percent
                            },
                        }
                    }
                };

                self.replace_constraint(&Key(KeyId::Entity(entity), "width"), constraint);

                let upper_bound_key = Key(KeyId::Entity(entity), "upper_bound");
                let lower_bound_key = Key(KeyId::Entity(entity), "lower_bound");
                let height_key = Key(KeyId::Entity(entity), "height");

                let upper_bound = self.fill_variable(&upper_bound_key, None);
                let lower_bound = self.fill_variable(&lower_bound_key, None);
                let constraint = match bound.height.clone().unwrap_or(Coordinate::Pixel(100.0)) {
                    Coordinate::Pixel(units) => {
                        let height = self.fill_variable(&height_key, Some(MEDIUM));
                        self.solver.suggest_value(height, units);

                        let height_constraint = lower_bound - upper_bound |EQ(WEAK)| height;
                        height_constraint
                    },
                    Coordinate::Percent(percent) => {
                        match class.parents.get(entity) {
                            Some(parent) => {
                                let parent_upper_bound = self.fill_variable(&Key(KeyId::Entity(parent.entity), "upper_bound"), None);
                                let parent_lower_bound = self.fill_variable(&Key(KeyId::Entity(parent.entity), "lower_bound"), None);
                                lower_bound - upper_bound |EQ(WEAK)| (parent_lower_bound - parent_upper_bound) * percent
                            },
                            None => {
                                let height = self.fill_variable(&Key(KeyId::Context, "viewport height"), Some(REQUIRED - 1.0));
                                lower_bound - upper_bound |EQ(WEAK)| height * percent
                            },
                        }
                    }
                };

                self.replace_constraint(&Key(KeyId::Entity(entity), "height"), constraint);
            }
        }

        let changes = self.solver.fetch_changes().iter().cloned().collect::<Vec<(Variable, f64)>>();
        let changes = changes.iter().map(|&(variable, change)| {
            (
                self.var_map.get(&variable)
                            .unwrap_or(&Key(KeyId::Unkeyed, "Unknown"))
                            .clone(),
                variable,
                change,
            )
        }).collect::<Vec<(Key, Variable, f64)>>();
        class.changes.set_changes(changes);
    }
}
