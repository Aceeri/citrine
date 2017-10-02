
extern crate citrine;
extern crate specs;

use specs::{Dispatcher, World};
use citrine::class::{AbsolutePosition, Viewport, Children, Coordinate, Parent, Position, Display, Bounds};

fn main() {
    let mut dispatcher = citrine::dispatcher();
    let mut world = World::new();
    world.add_resource::<Viewport>(Viewport {
        width: 1920,
        height: 1080,
    });
    world.register::<Parent>();
    world.register::<Display>();
    world.register::<Children>();
    world.register::<Position>();
    world.register::<AbsolutePosition>();
    world.register::<Bounds>();

    let entity = world.create_entity()
        .with(Position {
            x: Some(Coordinate::Pixel(50.0)),
            y: Some(Coordinate::Percent(0.5)),
            .. Position::default()
        })
        .with(AbsolutePosition::default())
        .with(Bounds {
            width: Some(Coordinate::Percent(1.0)), 
            height: Some(Coordinate::Percent(0.01)), 
        })
        .build();

    let child_entity = world.create_entity()
        .with(Position::default())
        .with(AbsolutePosition::default())
        .with(Bounds {
            width: Some(Coordinate::Percent(0.70)), 
            height: Some(Coordinate::Pixel(100.0)), 
        })
        .build();

    dispatcher.dispatch(&mut world.res);
    world.maintain();
    dispatcher.dispatch(&mut world.res);
    world.maintain();
    dispatcher.dispatch(&mut world.res);
    world.maintain();
}
