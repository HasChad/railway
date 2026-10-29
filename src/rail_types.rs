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

#[derive(Component, Debug, Clone)]
pub struct PlacedRail {
    pub kind: PieceKind,
    pub position: Vec2,
    pub rotation_deg: f32,
    pub ports: Vec<Port>,
    pub path: Vec<Vec2>,
    pub sprite: Handle<Image>,
}

impl PlacedRail {
    pub fn straight(position: Vec2, asset_server: &AssetServer) -> PlacedRail {
        PlacedRail {
            kind: PieceKind::Straight,
            position,
            rotation_deg: 0.0,
            ports: vec![
                Port::new(Vec2::new(-50.0, 0.0), -90.0),
                Port::new(Vec2::new(50.0, 0.0), 90.0),
            ],
            path: vec![Vec2::new(-50.0, 0.0), Vec2::new(50.0, 0.0)],
            sprite: asset_server.load("rail.png"),
        }
    }
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
