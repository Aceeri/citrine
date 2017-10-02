
use std::any::Any;

use specs::{Component, FlaggedStorage, DenseVecStorage, Entity};
//use ::track::TrackStorage;

macro_rules! define_component {
    ( $ident:ident ) => {
        impl Component for $ident {
            type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

/// Describes the layout for this ui element.
///
/// All children will behave by these.
pub struct Layout(pub Box<Any + Send + Sync>);

pub struct Grid {
    columns: Option<usize>,
    column_stretch: Vec<f32>,
    rows: Option<usize>,
    row_stretch: Vec<f32>,
}

#[derive(Clone, Debug, Default)]
pub struct Display {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

#[derive(Clone, Debug)]
pub enum Coordinate {
    /// Coordinate space in pixels.
    Pixel(f32),
    /// Coordinate space in percentage of parent.
    Percent(f32),
}

/// Text to be displayed in this segment.
pub struct Text {
    pub text: String,
    // TODO: Lots of formatting elements.
    // pub font: String,
    // pub size: u32,
    // pub wrap: bool,
}

/// Defines the UI entity that is the parent of this
/// UI section.
#[derive(Clone, Debug)]
pub struct Parent {
    /// Defines another `Entity` as the parent UI of this one.
    pub entity: Entity,
}

/// Defines the children of this UI entity.
///
/// Used for things like layout constraints.
#[derive(Clone, Debug, Default)]
pub struct Children {
    entities: Vec<Entity>,
}

impl Children {
    pub fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }

    pub(crate) fn push(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}

/// Position of the UI section.
///
/// Top/left/bottom/right are the relations to the parent's bounds.
#[derive(Clone, Debug, Default)]
pub struct Position {
    /// How the position behaves.
    pub kind: PositionKind,
    /// Stretches to the top of the parent.
    pub x: Option<Coordinate>,
    /// Stretches to the left.
    pub y: Option<Coordinate>,
    /// Z-ordering of UI.
    pub z: Option<usize>,
}

/// Type of positioning. Default is `Relative`.
#[derive(Clone, Debug)]
pub enum PositionKind {
    /// Positions in the parent's dimensions without regard to other portions of the UI.
    Free,
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
    Absolute,
}

impl Default for PositionKind {
    fn default() -> Self {
        PositionKind::Absolute
    }
}

#[derive(Clone, Debug, Default)]
pub struct Bounds {
    pub width: Option<Coordinate>,
    pub height: Option<Coordinate>,
}

/// The computed result of the `Position` and `Bounds` components.
#[derive(Clone, Debug, Default)]
pub struct AbsolutePosition {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    /// Z-ordering.
    pub z: usize,
}

// Component quick definitions
define_component!(Parent);
define_component!(Text);
define_component!(Layout);
define_component!(Display);
define_component!(Children);
define_component!(Position);
define_component!(AbsolutePosition);
define_component!(Bounds);
