
use specs::{Component, Entity, LazyUpdate, World};

use class::Text;

pub trait UiBuilder {
    fn with<C>(self, c: C) -> Self where C: Component + Send + Sync;
    fn with_id<C>(self, c: C, id: usize) -> Self where C: Component + Send + Sync;
    fn text(self, s: String) -> Self;
    fn children<'a>(self, list: &'a [Entity]) -> Self;

    fn done(self) -> Entity;
}

pub trait InsertComponent {
    fn get_insert<T>(&mut self, value: T, id: usize)
        where T: Component + Send + Sync;
    fn entity(&self) -> Entity;
}

pub struct Ui<'a> {
    world: &'a mut World,
    entity: Entity,
}

impl<'a> Ui<'a> {
    pub fn new(world: &'a mut World, entity: Entity) -> Self {
        Ui {
            world: world,
            entity: entity,
        }
    }
}

impl<'a> InsertComponent for Ui<'a> {
    fn get_insert<T>(&mut self, value: T, id: usize)
        where T: Component + Send + Sync,
    {
        self.world.write_with_id(id).insert(self.entity, value);
    }
    fn entity(&self) -> Entity {
        self.entity
    }
}

pub struct LazyUi<'a> {
    lazy: &'a LazyUpdate,
    entity: Entity,
}

impl<'a> LazyUi<'a> {
    pub fn new(lazy: &'a mut LazyUpdate, entity: Entity) -> Self {
        LazyUi {
            lazy: lazy,
            entity: entity,
        }
    }
}

impl<'a> InsertComponent for LazyUi<'a> {
    fn get_insert<T>(&mut self, value: T, id: usize)
        where T: Component + Send + Sync,
    {
        let entity = self.entity.clone();
        self.lazy.execute(move |world| {
            world.write_with_id::<T>(id).insert(entity, value);
        })
    }
    fn entity(&self) -> Entity {
        self.entity
    }
}

impl<T> UiBuilder for T
    where T: InsertComponent + Send + Sync,
{
    fn with<C>(mut self, value: C) -> Self
        where C: Component + Send + Sync,
    {
        self.get_insert::<C>(value, 0);
        self
    }
    fn with_id<C>(mut self, value: C, id: usize) -> Self
        where C: Component + Send + Sync,
    {
        self.get_insert::<C>(value, id);
        self
    }
    fn text(mut self, string: String) -> Self {
        self.get_insert::<Text>(Text { text: string, }, 0);
        self
    }
    fn children<'a>(mut self, children: &'a [Entity]) -> Self {
        self
    }
    fn done(self) -> Entity {
        self.entity()
    }
}
