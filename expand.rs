#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;

extern crate cassowary;
extern crate specs;
#[macro_use]
extern crate shred_derive;
extern crate shred;

pub mod class {
    //pub mod section;
    //pub mod page;

    use specs::{Component, DenseVecStorage, Entity, FlaggedStorage};
    macro_rules! define_component(( $ ident : ident ) => {
                                  impl Component for $ ident {
                                  type Storage = FlaggedStorage < Self ,
                                  DenseVecStorage < Self >> ; } });
    /// Defines the UI entity that is the parent of this
    /// UI section.
    pub struct Parent {
        entity: Entity,
    }
    /// Position of the UI section.
    ///
    /// Top/left/bottom/right are the relations to the parent's bounds.
    pub struct Position {
        pub kind: PositionKind,
        pub top: Option<f32>,
        pub left: Option<f32>,
        pub bottom: Option<f32>,
        pub right: Option<f32>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::default::Default for Position {
        #[inline]
        fn default() -> Position {
            Position{kind: ::std::default::Default::default(),
                     top: ::std::default::Default::default(),
                     left: ::std::default::Default::default(),
                     bottom: ::std::default::Default::default(),
                     right: ::std::default::Default::default(),}
        }
    }
    /// Type of positioning. Default is `Normal`.
    ///
    pub enum PositionKind { Absolute, Relative, Normal, }
    impl Default for PositionKind {
        fn default() -> Self { PositionKind::Relative }
    }
    pub struct Bounds {
        pub width: Option<f32>,
        pub height: Option<f32>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::default::Default for Bounds {
        #[inline]
        fn default() -> Bounds {
            Bounds{width: ::std::default::Default::default(),
                   height: ::std::default::Default::default(),}
        }
    }
    impl Component for Parent {
        type
        Storage
        =
        FlaggedStorage<Self, DenseVecStorage<Self>>;
    }
    impl Component for Position {
        type
        Storage
        =
        FlaggedStorage<Self, DenseVecStorage<Self>>;
    }
    impl Component for Bounds {
        type
        Storage
        =
        FlaggedStorage<Self, DenseVecStorage<Self>>;
    }
}
pub mod solver {
    use std::collections::HashMap;
    use cassowary::{Solver, Variable};
    use specs::{Component, Entity, Entities, Fetch, Join, ReadStorage,
                System};
    use class::*;
    /// Just a macro to help with the abundant boilerplate related to getting a lot of
    /// components that are related to the UI and converting them into the `FlaggedStorage`s
    /// so we can just iterate over the flagged portions.
    macro_rules! class(( $ ( $ name : ident => $ component : ident , ) * ) =>
                       {
                       pub enum Id { $ ( $ component , ) * } # [
                       derive ( SystemData ) ] pub struct Class < 'a > {
                       $ ( $ name : ReadStorage < 'a , $ component > , ) * }
                       pub struct FlaggedClass < 'a > {
                       $ (
                       $ name : & 'a < $ component as Component > :: Storage ,
                       ) * } impl < 'a , 'b > From < & 'b Class < 'a >> for
                       FlaggedClass < 'b > {
                       fn from ( class : & 'b Class < 'a > ) -> Self {
                       FlaggedClass {
                       $ ( $ name : class . $ name . open (  ) . 1 , ) * } } }
                       });
    pub enum Id { Position, Bounds, }
    pub struct Class<'a> {
        positions: ReadStorage<'a, Position>,
        bounds: ReadStorage<'a, Bounds>,
    }
    impl <'a> ::shred::SystemData<'a> for Class<'a> where
     ReadStorage<'a, Position>: ::shred::SystemData<'a>,
     ReadStorage<'a, Bounds>: ::shred::SystemData<'a> {
        fn fetch(res: &'a ::shred::Resources, id: usize) -> Self {
            Class{positions: ::shred::SystemData::fetch(res, id),
                  bounds: ::shred::SystemData::fetch(res, id),}
        }
        fn reads(id: usize) -> Vec<::shred::ResourceId> {
            let mut r = Vec::new();
            {
                let mut reads =
                    <ReadStorage<'a, Position> as
                        ::shred::SystemData>::reads(id);
                r.append(&mut reads);
            }
            {
                let mut reads =
                    <ReadStorage<'a, Bounds> as
                        ::shred::SystemData>::reads(id);
                r.append(&mut reads);
            }
            r
        }
        fn writes(id: usize) -> Vec<::shred::ResourceId> {
            let mut r = Vec::new();
            {
                let mut writes =
                    <ReadStorage<'a, Position> as
                        ::shred::SystemData>::writes(id);
                r.append(&mut writes);
            }
            {
                let mut writes =
                    <ReadStorage<'a, Bounds> as
                        ::shred::SystemData>::writes(id);
                r.append(&mut writes);
            }
            r
        }
    }
    pub struct FlaggedClass<'a> {
        positions: &'a <Position as Component>::Storage,
        bounds: &'a <Bounds as Component>::Storage,
    }
    impl <'a, 'b> From<&'b Class<'a>> for FlaggedClass<'b> {
        fn from(class: &'b Class<'a>) -> Self {
            FlaggedClass{positions: class.positions.open().1,
                         bounds: class.bounds.open().1,}
        }
    }
    pub struct SolverSystem {
        solver: Solver,
        variables: HashMap<Entity, HashMap<Id, Variable>>,
    }
    impl Default for SolverSystem {
        fn default() -> Self {
            SolverSystem{solver: Solver::new(), variables: HashMap::new(),}
        }
    }
    impl <'a> System<'a> for SolverSystem {
        type
        SystemData
        =
        (Entities<'a>, Class<'a>);
        fn run(&mut self, (entities, class): Self::SystemData) {
            {
                let flagged = FlaggedClass::from(&class);
                for (entity, parent) in (&*entities, &flagged.parents).join()
                    {
                }
            }
        }
    }
}
