use bevy::prelude::*;

pub const SPACING: f32 = 50.0;

pub fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // let placed = PlacedRail {
    //     kind: PieceKind::Straight,
    //     position: Vec2::ZERO,
    //     rotation_deg: 0.0,
    //     ports: vec![
    //         Port::new(Vec2::new(25.0, 0.0), 0.0),
    //         Port::new(Vec2::new(-25.0, 0.0), 0.0),
    //     ],
    // };

    // commands.spawn((
    //     placed.clone(),
    //     Transform::from_translation(placed.position.extend(0.0)),
    //     Sprite::from_image(asset_server.load("rail.png")),
    // ));

    // commands.spawn((
    //     Train {
    //         current_edge: 0,
    //         progress: 0.0,
    //         speed: 0.0,
    //     },
    //     Transform::from_xyz(0.0, 0.0, 10.0),
    //     Sprite::from_image(asset_server.load("wagon.png")),
    // ));
}
