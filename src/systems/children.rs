
use specs::{Entities, Join, System, ReadStorage, WriteStorage};

use class::{Children, Parent};

pub struct ChildrenSystem;
impl<'a> System<'a> for ChildrenSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Parent>,
        WriteStorage<'a, Children>,
    );
    fn run(&mut self, (entities, parents, mut children): Self::SystemData) {
        let flagged_parents = parents.open().1;

        {
            // If an entity is a parent, then it should have children.
            for (entity, parent) in (&*entities, flagged_parents).join() {
                if let None = children.get(parent.entity) {
                    children.insert(parent.entity, Children::default());
                }
            }

            // Iterate over changed parents and add the entities to that parents child list.
            for (entity, parent) in (&*entities, flagged_parents).join() {
                let mut child_list = children.get_mut(parent.entity).unwrap();
                child_list.push(entity);
            }
        }

        {
            let mut flagged_children = (&mut children).open().1;

            // Clean up children that are no longer parented to this UI.
            for (entity, mut children) in (&*entities, flagged_children).join() {
                let mut remove = Vec::new();
                for (index, child) in children.entities().iter().enumerate() {
                    match parents.get(*child) {
                        Some(parent) if parent.entity != entity => { },
                        _ => remove.push(index),
                    }
                }
            }
        }

        for (entity, child) in (&*entities, &children).join() {
            println!("{:?} contains {:?}", entity, child.entities());
        }
    }
}
