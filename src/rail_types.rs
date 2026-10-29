use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Port {
    pub offset: Vec2,
    pub facing_deg: f32,
}

impl Port {
    pub fn new(offset: Vec2, facing_deg: f32) -> Self {
        Self { offset, facing_deg }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceKind {
    Straight,
    // Curve,
    // HalfDiagonalStraight,
    // Split,
}

#[derive(Debug, Clone)]
pub struct RailPieceType {
    pub kind: PieceKind,
    pub position: Vec2,
    pub rotation_deg: f32,
    pub ports: Vec<Port>,
    pub path: Vec<Vec2>, // polyline / bezier control points
    pub sprite: Handle<Image>,
}

#[derive(Component, Debug, Clone)]
pub struct PlacedRail {
    pub kind: PieceKind,
    pub position: Vec2,
    pub rotation_deg: f32,
    pub ports: Vec<Port>,
}

#[derive(Component, Debug, Clone)]
pub struct Train {
    pub current_edge: usize, // index into RailNetwork.edges, or your own path repr
    pub progress: f32,       // 0.0 to 1.0 along the current edge
    pub speed: f32,
}

#[derive(Component, Debug, Clone)]
pub struct RailNode {
    pub nodes: Vec<Vec2>,
}
