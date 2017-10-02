
use specs::{Component, System, Entities, Fetch, WriteStorage, ReadStorage, Join};

use hibitset::{BitSetLike, BitSetOr};

use class::{Coordinate, Position, PositionKind, Bounds, AbsolutePosition, Viewport};

fn relative_parent(parent: f32, coordinate: Coordinate) -> f32 {
    match coordinate {
        Coordinate::Percent(percent) => parent as f32 * percent,
        Coordinate::Pixel(pixel) => pixel,
    }
}

/// Solver for computing positions and bounds into the absolute position that the UI
/// will be on the screen.
pub struct PositionSystem;
impl<'a> System<'a> for PositionSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, Viewport>,

        WriteStorage<'a, Position>,
        WriteStorage<'a, Bounds>,
        WriteStorage<'a, AbsolutePosition>,
    );
    fn run(&mut self, mut data: Self::SystemData) {
        let (entities, viewport, mut positions, mut bounds, mut absolutes) = data;

        // Frame has gone by, should clear the absolute's flags.
        (&mut absolutes).open().1.clear_flags();

        // Did either the position or the bounds change?
        let filter = {
            let positions_mask = positions.open().1.open().0;
            let bounds_mask = bounds.open().1.open().0;
            BitSetOr(positions_mask.clone(), bounds_mask.clone())
        };

        for (entity, _, absolute) in (&*entities, &filter, &mut absolutes).join() {
            let (x, y) = match positions.get(entity) {
                Some(position) => match position.kind { 
                    PositionKind::Free => unimplemented!(),
                    PositionKind::Relative => unimplemented!(),
                    PositionKind::Absolute => {
                        let x = relative_parent(viewport.width as f32, position.x.clone().unwrap_or(Coordinate::Pixel(0.0)));
                        let y = relative_parent(viewport.height as f32, position.y.clone().unwrap_or(Coordinate::Pixel(0.0)));
                        (x, y)
                    },
                },
                None => (0.0, 0.0),
            };

            let (width, height) = {
                match bounds.get(entity) {
                    Some(bounds) => {
                        let width = relative_parent(viewport.width as f32, bounds.width.clone().unwrap_or(Coordinate::Pixel(100.0)));
                        let height = relative_parent(viewport.height as f32, bounds.height.clone().unwrap_or(Coordinate::Pixel(100.0)));
                        (width, height)
                    },
                    None => (100.0, 100.0),
                }
            };

            absolute.x = x;
            absolute.y = y;
            absolute.width = width;
            absolute.height = height;

            println!("{:?} = {:?}", entity, absolute);
        }

        // Reset the flags.
        (&mut positions).open().1.clear_flags();
        (&mut bounds).open().1.clear_flags();
    }
}
