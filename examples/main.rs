
extern crate citrine;
extern crate specs;

use specs::{Dispatcher, World};
use citrine::class::{Coordinate, Parent, Position, Bounds};

fn main() {
    let mut dispatcher = citrine::dispatcher();
    let mut world = World::new();
    world.register::<Parent>();
    world.register::<Position>();
    world.register::<Bounds>();

    let entity = world.create_entity()
        .with(Position::default())
        .with(Bounds {
            width: Some(Coordinate::Percent(1.0)), 
            height: Some(Coordinate::Percent(1.0)), 
        })
        .build();

    let child_entity = world.create_entity()
        .with(Parent { entity: entity })
        .with(Position::default())
        .with(Bounds {
            width: Some(Coordinate::Percent(0.70)), 
            height: Some(Coordinate::Pixel(100.0)), 
        })
        .build();

    dispatcher.dispatch(&mut world.res);
    world.maintain();
}
