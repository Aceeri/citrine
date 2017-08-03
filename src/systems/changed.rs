
use specs::{Entities, Fetch, Join, System, ReadStorage, WriteStorage};

use ::solver::{Changes, Key, KeyId};
use ::class::{Positional};

pub struct ChangedSystem;
impl<'a> System<'a> for ChangedSystem {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, Changes>,
        WriteStorage<'a, Positional>,
    );
    fn run(&mut self, (entities, changes, mut positions): Self::SystemData) {
        println!("Changes");
        for &(ref key, ref variable, change) in changes.changes() {
            println!("  {:?}", (&key, &variable, &change));
            match *key {
                Key(KeyId::Entity(entity), "left_bound") => {
                    if let Some(position) = positions.get_mut(entity) {
                        position.left = change;
                    }
                },
                Key(KeyId::Entity(entity), "right_bound") => {
                    if let Some(position) = positions.get_mut(entity) {
                        position.right = change;
                    }
                },
                Key(KeyId::Entity(entity), "upper_bound") => {
                    if let Some(position) = positions.get_mut(entity) {
                        position.top = change;
                    }
                },
                Key(KeyId::Entity(entity), "lower_bound") => {
                    if let Some(position) = positions.get_mut(entity) {
                        position.bottom = change;
                    }
                },
                _ => { }
            }
        }
    }
}
