
use specs::{Component, System, Entities, WriteStorage, ReadStorage, Join};

use hibitset::BitSetOr;

use class::{Position, Bounds, AbsolutePosition};

/// Solver for computing positions and bounds into the absolute position that the UI
/// will be on the screen.
///
/// Anything modifying positions and bounds of the UI should be run before this system,
/// otherwise the changes will not trigger a redraw of the UI's position.
pub struct PositionSystem;
impl<'a> System<'a> for PositionSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Bounds>,
        WriteStorage<'a, AbsolutePosition>,
    );
    fn run(&mut self, mut data: Self::SystemData) {
        let (entities, mut positions, mut bounds, mut absolutes) = data;

        // No contention
        for (position, _, absolutes) in (&positions, !&bounds, &mut absolutes).join() {
            
        }

        // 
        for (_, bounds, absolutes) in (!&positions, &bounds, &mut absolutes).join() {
            
        }

        /*
        let filter = {
            let (flag_positions_mask, mut flag_positions) = positions.open();
            let (flag_bounds_mask, mut flag_bounds) = bounds.open();
            BitSetOr(flag_positions_mask.clone(), flag_bounds_mask.clone())
        };

        for (_, position, bounds, absolutes) in (&filter, &positions, &bounds, &mut absolutes).join() {
             
        }
        */

        // Reset the flags.
        {
            (&mut positions).open().1.clear_flags();
            (&mut bounds).open().1.clear_flags();
        }
    }
}
