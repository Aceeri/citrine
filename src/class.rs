
#[derive(Default)]
pub struct Position {
    pub kind: PositionKind,
    pub top: Option<f32>,
    pub left: Option<f32>,
    pub bottom: Option<f32>,
    pub right: Option<f32>,
}

pub enum PositionKind {
    Absolute,
    Relative,
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

#[derive(Default)]
pub struct Class {
    pub id: Option<String>,
    pub position: Position,
    pub bounds: Bounds,
}

