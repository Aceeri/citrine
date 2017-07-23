
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use cassowary::{Constraint, Solver, Variable};
use cassowary::strength::{WEAK, MEDIUM, STRONG, REQUIRED};
use cassowary::WeightedRelation::{self, LE, EQ, GE};
use specs::{Component, Entity, Entities, Fetch, Join, ReadStorage, System};

use ::class::*;

type Key = (Entity, Id, &'static str);

/// Just a macro to help with the abundant boilerplate related to getting a lot of
/// components that are related to the UI and converting them into the `FlaggedStorage`s
/// so we can just iterate over the flagged portions.
macro_rules! class {
    ( $( $name:ident => $component:ident, )* ) => {
        #[derive(PartialEq, Eq, Hash, Clone)]
        pub enum Id {
            $(
                $component,
            )*
        }

        #[derive(SystemData)]
        pub struct ClassData<'a> {
            $(
                $name: ReadStorage<'a, $component>,
            )*
        }

        pub struct FlaggedClass<'a> {
            $(
                $name: &'a <$component as Component>::Storage,
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
    }

    fn viewport(&mut self, width: f64, height: f64) {
        self.solver.suggest_value(self.viewport[0], width);
        self.solver.suggest_value(self.viewport[1], height);
    }

    fn has_variable(&self, key: &Key) -> bool {
        self.key_map.contains_key(key)
    }

    /// Fills in the variable if it doesnt currently exist in the system.
    fn fill_variable(&mut self, key: &Key) -> Variable {
        match self.key_map.entry(key.clone()) {
            Entry::Occupied(occupied) => occupied.get().clone(),
            Entry::Vacant(vacant) => {
                let var = Variable::new();
                vacant.insert( var.clone() );
                self.var_map.insert( var.clone(), key.clone() );
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
}

impl<'a> System<'a> for SolverSystem {
    type SystemData = (Entities<'a>, ClassData<'a>);
    fn run(&mut self, (entities, class): Self::SystemData) {
        {
            let flagged = FlaggedClass::from(&class);

            for (entity, parent, position) in (&*entities, flagged.parents, &class.positions).join() {
                match position.kind {
                    // Set left/right/upper/lower bounds for the UI.
                    PositionKind::Absolute |
                    PositionKind::Relative => {
                        if let Some(parent_position) = class.positions.get(parent.entity) {
                            let mut bound = |string: &'static str, relation: WeightedRelation | {
                                let key = (entity, Id::Position, string);
                                let parent_key = (parent.entity, Id::Position, string);
                                let var = self.fill_variable(&key);
                                let parent_var = self.fill_variable(&parent_key);

                                let constraint = var |relation| parent_var;
                                self.replace_constraint(&key, constraint);
                            };

                            bound("left_bound", GE(REQUIRED));
                            bound("right_bound", LE(REQUIRED));
                            bound("upper_bound", GE(REQUIRED));
                            bound("lower_bound", LE(REQUIRED));
                        }
                    },
                    PositionKind::Free => { }, // Do nothing.
                }
            }

            // TODO: Set up variables and constraints for positioning UI rectangles properly
            for (entity, position) in (&*entities, flagged.positions).join() {
                
            }

            // TODO: Set up variables and constraints for bounds (width and height).
            for (entity, bound) in (&*entities, flagged.bounds).join() {
                
            }
        }
    }
}
