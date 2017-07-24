
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

use cassowary::{Constraint, Solver, Variable};
use cassowary::strength::{WEAK, MEDIUM, STRONG, REQUIRED};
use cassowary::WeightedRelation::{self, LE, EQ, GE};
use specs::{Component, Entity, Entities, Fetch, Join, ReadStorage, System, WriteStorage};

use ::class::*;

type Key = (Entity, &'static str);

/// Just a macro to help with the abundant boilerplate related to getting a lot of
/// components that are related to the UI and converting them into the `FlaggedStorage`s
/// so we can just iterate over the flagged portions.
macro_rules! class {
    ( $( $name:ident => $component:ident, )* ) => {
        /*
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub enum Id {
            Viewport,
            $(
                $component,
            )*
        }
        */

        #[derive(SystemData)]
        pub struct ClassData<'a> {
            $(
                $name: ReadStorage<'a, $component>,
            )*
        }

        #[derive(SystemData)]
        pub struct ClassDataMut<'a> {
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

        #[derive(Default)]
        pub struct ClassSet {
            $(
                $name: HashSet<Entity>,
            )*
        }

        impl ClassSet {
            pub fn clear_entity(&mut self, entity: &Entity) {
                $(
                    self.$name.remove(entity);
                )*
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
    parents => Parent,
    positions => Position,
    bounds => Bounds,
);

pub struct SolverSystem {
    solver: Solver,

    // Variables
    viewport: [Variable; 2], // width, height

    key_map: HashMap<Key, Variable>,
    var_map: HashMap<Variable, Key>,

    constraints: HashMap<Key, Constraint>,
}

impl Default for SolverSystem {
    fn default() -> Self {
        let mut system = SolverSystem {
            solver: Solver::new(),
            viewport: [Variable::new(), Variable::new()],

            key_map: HashMap::new(),
            var_map: HashMap::new(),
            constraints: HashMap::new(),
        };

        system.setup();
        system
    }
}

impl SolverSystem {
    /// Resets the solver and re-adds the necessary variables.
    fn setup(&mut self) {
        self.solver.reset();
        
        // Viewport Variables
        self.solver.add_edit_variable(self.viewport[0], REQUIRED - 1.0);
        self.solver.add_edit_variable(self.viewport[1], REQUIRED - 1.0);
        self.suggest_viewport(300.0f64, 300.0f64);
    }

    fn suggest_viewport(&mut self, width: f64, height: f64) {
        self.solver.suggest_value(self.viewport[0], width);
        self.solver.suggest_value(self.viewport[1], height);
    }

    fn has_variable(&self, key: &Key) -> bool {
        self.key_map.contains_key(key)
    }

    /// Fills in the variable if it doesnt currently exist in the system.
    fn fill_variable(&mut self, key: &Key, strength: Option<f64>) -> Variable {
        match self.key_map.entry(key.clone()) {
            Entry::Occupied(occupied) => occupied.get().clone(),
            Entry::Vacant(vacant) => {
                let var = Variable::new();
                vacant.insert( var.clone() );
                self.var_map.insert( var.clone(), key.clone() );
                if let Some(strength) = strength {
                    self.solver.add_edit_variable(var, strength);
                }
                var
            }
        }
    }

    fn has_constraint(&self, key: &Key) -> bool {
        self.constraints.contains_key(key)
    }

    fn replace_constraint(&mut self, key: &Key, constraint: Constraint) {
        match self.constraints.get_mut(key) {
            Some(old_constraint) => {
                if &*old_constraint != &constraint {
                    self.solver.remove_constraint(&old_constraint);
                }
            },
            _ => { },
        }

        self.solver.add_constraint(constraint.clone());
        self.constraints.insert(key.clone(), constraint);
    }

    fn print_variable(&self, variable: &Variable) {
        if variable == &self.viewport[0] {
            print!("Viewport Width:");
        }

        if variable == &self.viewport[1] {
            print!("Viewport Height:");
        }

        if let Some(key) = self.var_map.get(variable) {
            print!("{:?}", key);
        }
    }
}

impl<'a> System<'a> for SolverSystem {
    type SystemData = (Entities<'a>, ClassData<'a>);
    fn run(&mut self, (entities, class): Self::SystemData) {
        {
            let flagged = FlaggedClass::from(&class);

            println!("Alive:");
            for entity in (&*entities).join() {
                println!("  {:?}", entity);
            }

            for (entity, parent, position) in (&*entities, flagged.parents, &class.positions).join() {
                match position.kind {
                    // Set left/right/upper/lower bounds for the UI.
                    PositionKind::Absolute |
                    PositionKind::Relative => {
                        let mut bound = |string: &'static str, relation: WeightedRelation | {
                            let key = (entity, string);
                            let parent_key = (parent.entity, string);
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

            for (entity, _, position) in (&*entities, !&class.parents, &class.positions).join() {
                match position.kind {
                    // Set left/right/upper/lower bounds for the UI.
                    PositionKind::Absolute |
                    PositionKind::Relative => {
                        let key = (entity, "left_bound");
                        let var = self.fill_variable(&key, None);
                        let constraint = var |GE(REQUIRED)| 0.0;
                        self.replace_constraint(&key, constraint);

                        let key = (entity, "right_bound");
                        let var = self.fill_variable(&key, None);
                        let constraint = var |LE(REQUIRED)| self.viewport[0];
                        self.replace_constraint(&key, constraint);

                        let key = (entity, "upper_bound");
                        let var = self.fill_variable(&key, None);
                        let constraint = var |GE(REQUIRED)| 0.0;
                        self.replace_constraint(&key, constraint);

                        let key = (entity, "lower_bound");
                        let var = self.fill_variable(&key, None);
                        let constraint = var |LE(REQUIRED)| self.viewport[1];
                        self.replace_constraint(&key, constraint);
                    },
                    PositionKind::Free => { }, // Do nothing.
                }
            }

            // TODO: Set up variables and constraints for positioning UI rectangles properly
            for (entity, position) in (&*entities, flagged.positions).join() {
                
            }

            for (entity, bound) in (&*entities, flagged.bounds).join() {
                let left_bound_key = (entity, "left_bound");
                let right_bound_key = (entity, "right_bound");
                let width_key = (entity, "width");

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
                                let parent_left_bound = self.fill_variable(&(parent.entity, "left_bound"), None);
                                let parent_right_bound = self.fill_variable(&(parent.entity, "right_bound"), None);
                                right_bound - left_bound |EQ(WEAK)| (parent_right_bound - parent_left_bound) * percent
                            },
                            None => {
                                right_bound - left_bound |EQ(WEAK)| self.viewport[0] * percent
                            },
                        }
                    }
                };

                self.replace_constraint(&(entity, "width"), constraint);

                let upper_bound_key = (entity, "upper_bound");
                let lower_bound_key = (entity, "lower_bound");
                let height_key = (entity, "height");

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
                                let parent_upper_bound = self.fill_variable(&(parent.entity, "upper_bound"), None);
                                let parent_lower_bound = self.fill_variable(&(parent.entity, "lower_bound"), None);
                                lower_bound - upper_bound |EQ(WEAK)| (parent_lower_bound - parent_upper_bound) * percent
                            },
                            None => {
                                lower_bound - upper_bound |EQ(WEAK)| self.viewport[0] * percent
                            },
                        }
                    }
                };

                self.replace_constraint(&(entity, "height"), constraint);
            }
        }

        let changes = self.solver.fetch_changes().iter().cloned().collect::<Vec<(Variable, f64)>>();
        println!("Changed:");
        for change in changes {
            print!("  ");
            self.print_variable(&change.0);
            println!(" {:?}", change.1);
        }
    }
}
