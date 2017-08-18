
extern crate cassowary;
extern crate specs;
#[macro_use]
extern crate shred_derive;
extern crate shred;
extern crate hibitset;

use specs::{Dispatcher, DispatcherBuilder};

pub mod systems;
pub mod class;
pub mod solver;
pub mod ui;
mod track;

pub fn dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    let solver = solver::SolverSystem::default();
    let mut dispatcher = DispatcherBuilder::new()
        .add(systems::changed::ChangedSystem, "changed", &[])
        .add(systems::children::ChildrenSystem, "children", &[])
        // ...
        // Modifications to the class components should go here.
        // ...
        // Unfortunate as `cassowary-rs` uses `Rc`s, but fine since it needs to run at the end anyways.
        .add_thread_local(solver)
        .add_thread_local(solver::ResetSystem)
        .build();

    dispatcher
}

