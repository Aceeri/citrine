
use specs::{Component, DenseVecStorage, Entity, FlaggedStorage};

macro_rules! define_component {
    ( $ident:ident ) => {
        impl Component for $ident {
            type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
        }
    }
}

pub enum Coordinate {
    /// Coordinate space in pixels.
    Pixel(u32),
    /// Coordinate space in percentage of parent.
    Percent(f64),
}

/// Defines the UI entity that is the parent of this
/// UI section.
pub struct Parent {
    /// Defines another `Entity` as the parent UI of this one.
    pub entity: Entity,
}

/// Position of the UI section.
///
/// Top/left/bottom/right are the relations to the parent's bounds.
#[derive(Default)]
pub struct Position {
    /// How the position behaves.
    pub kind: PositionKind,
    /// Stretches to the top of the parent.
    pub top: Option<Coordinate>,
    /// Stretches to the left.
    pub left: Option<Coordinate>,
    /// Stretches to the bottom.
    pub bottom: Option<Coordinate>,
    /// Stretches to the right.
    pub right: Option<Coordinate>,
}

/// Type of positioning. Default is `Relative`.
pub enum PositionKind {
    /// Positions in the parent's dimensions without regard to other portions of the UI.
    Absolute,
    /// Positions relative to its normal spot in the layout.
    /// `top`/`left`/`bottom`/`right` properties of the `Position` are preferred over the
    /// `width` and `height` of the `Bounds`.
    ///
    /// This is the default and should be used for the majority of UI since it keeps the
    /// layout from overlapping and allows for better resizing.
    Relative,
    /// Doesn't care about parent, just positions where you tell it to on the screen.
    /// `top`/`left`/`bottom`/`right` properties of the `Position` are discarded in favor
    /// of the `width` and `height` of the `Bounds`.
    ///
    /// Useful for things like floating windows and such.
    Free,
}

impl Default for PositionKind {
    fn default() -> Self {
        PositionKind::Relative
    }
}

#[derive(Default)]
pub struct Bounds {
    pub width: Option<Coordinate>,
    pub height: Option<Coordinate>,
}

// Component quick definitions
define_component!(Parent);
define_component!(Position);
define_component!(Bounds);
