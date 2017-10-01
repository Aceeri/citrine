
extern crate specs;
extern crate hibitset;

use specs::{Dispatcher, DispatcherBuilder};

pub mod systems;
pub mod class;
pub mod ui;
mod track;

pub fn dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    let mut dispatcher = DispatcherBuilder::new()
        //.add(systems::changed::ChangedSystem, "changed", &[])
        //.add(systems::children::ChildrenSystem, "children", &[])
        .add(systems::position::PositionSystem, "citrine/position", &[])
        .build();

    dispatcher
}

