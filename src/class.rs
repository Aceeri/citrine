
use specs::{Component, DenseVecStorage, Entity, FlaggedStorage};

macro_rules! define_component {
    ( $ident:ident ) => {
        impl Component for $ident {
            type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
        }
    }
}

/// Defines the UI entity that is the parent of this
/// UI section.
pub struct Parent {
    entity: Entity,
}

/// Position of the UI section.
///
/// Top/left/bottom/right are the relations to the parent's bounds.
#[derive(Default)]
pub struct Position {
    pub kind: PositionKind,
    pub top: Option<f32>,
    pub left: Option<f32>,
    pub bottom: Option<f32>,
    pub right: Option<f32>,
}

/// Type of positioning. Default is `Normal`.
///
pub enum PositionKind {
    Absolute,
    Relative,
    Normal,
}

impl Default for PositionKind {
    fn default() -> Self {
        PositionKind::Relative
    }
}

#[derive(Default)]
pub struct Bounds {
    pub width: Option<f32>,
    pub height: Option<f32>,
}

define_component!(Parent);
define_component!(Position);
define_component!(Bounds);
