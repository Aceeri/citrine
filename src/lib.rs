
extern crate cassowary;
extern crate specs;
#[macro_use]
extern crate shred_derive;
extern crate shred;

use specs::{Dispatcher, DispatcherBuilder};

pub mod class;
pub mod solver;

pub fn dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    let solver = solver::SolverSystem::default();
    let mut dispatcher = DispatcherBuilder::new()
        // Reset all the flags for the class components.
        //        .add_barrier()
        // ...
        // Modifications to the class components should go here.
        // ...
        // Unfortunate as `cassowary-rs` uses `Rc`s, but fine since it needs to run at the end anyways.
        .add_thread_local(solver)
        .add_thread_local(solver::ResetSystem)
        .build();

    dispatcher
}

