
use std::collections::HashMap;

use cassowary::{Solver, Variable};
use specs::{Component, Entity, Entities, Fetch, Join, ReadStorage, System};

use ::class::*;

/// Just a macro to help with the abundant boilerplate related to getting a lot of
/// components that are related to the UI and converting them into the `FlaggedStorage`s
/// so we can just iterate over the flagged portions.
macro_rules! class {
    ( $( $name:ident => $component:ident, )* ) => {
        #[derive(SystemData)]
        pub struct Class<'a> {
            $(
                $name: ReadStorage<'a, $component>,
            )*
        }

        pub struct FlaggedClass<'a> {
            $(
                $name: &'a <$component as Component>::Storage,
            )*
        }

        impl<'a, 'b> From<&'b Class<'a>> for FlaggedClass<'b> {
            fn from(class: &'b Class<'a>) -> Self {
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
    viewport: [Variable, Variable], // width, height
    variables: HashMap<Entity, Variable>,
}

impl Default for SolverSystem {
    fn default() -> Self {
        let mut system = SolverSystem {
            solver: Solver::new(),
            viewport: [Variable::new(), Variable::new()],
            variables: HashMap::new(),
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
        self.solver.add_edit_value(self.viewport[0]);
        self.solver.add_edit_value(self.viewport[1]);
    }

    fn suggest_viewport(&mut self, width: f64, height: f64) {
        self.solver.suggest_value(self.viewport.0, width);
        self.solver.suggest_value(self.viewport.1. height);
    }
}

impl<'a> System<'a> for SolverSystem {
    type SystemData = (Entities<'a>, Class<'a>);
    fn run(&mut self, (entities, class): Self::SystemData) {
        
        {
            let flagged = FlaggedClass::from(&class);

            // TODO: Set up variables and constraints for child parent relationships.
            for (entity, parent) in (&*entities, flagged.parents).join() {
                
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
