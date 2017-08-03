
use specs::{Component, DenseVecStorage, Entity};
use ::track::TrackStorage;

macro_rules! define_component {
    ( $ident:ident ) => {
        impl Component for $ident {
            type Storage = TrackStorage<Self, DenseVecStorage<Self>>;
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

pub struct Grid {
    columns: Option<usize>,
    column_stretch: Vec<f64>,

    rows: Option<usize>,
    row_stretch: Vec<f64>,
}

pub struct Celled {
    
}

pub struct List {
    
}

#[derive(Clone, Debug, Default)]
pub struct Positional {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

#[derive(Clone, Debug)]
pub enum Coordinate {
    /// Coordinate space in pixels.
    Pixel(f64),
    /// Coordinate space in percentage of parent.
    Percent(f64),
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
    pub top: Option<Coordinate>,
    /// Stretches to the left.
    pub left: Option<Coordinate>,
    /// Stretches to the bottom.
    pub bottom: Option<Coordinate>,
    /// Stretches to the right.
    pub right: Option<Coordinate>,
}

/// Type of positioning. Default is `Relative`.
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug, Default)]
pub struct Bounds {
    pub width: Option<Coordinate>,
    pub height: Option<Coordinate>,
}

// Component quick definitions
define_component!(Parent);
define_component!(Positional);
define_component!(Children);
define_component!(Position);
define_component!(Bounds);
