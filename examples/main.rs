
extern crate citrine;
extern crate specs;

use specs::{Dispatcher, World};
use citrine::class::{Parent, Position, Bounds};

fn main() {
    let mut dispatcher = citrine::dispatcher();
    let mut world = World::new();
    world.register::<Parent>();
    world.register::<Position>();
    world.register::<Bounds>();

    let entity = world.create_entity()
        .with(Position::default())
        .with(Bounds::default())
        .build();

    let child_entity = world.create_entity()
        .with(Parent { entity: entity })
        .with(Position::default())
        .with(Bounds::default())
        .build();

    dispatcher.dispatch(&mut world.res);
    world.maintain();
}
