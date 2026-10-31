use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    math::ops::powf,
    prelude::*,
};

use crate::{
    rail_types::{PlacedRail, RailNode},
    setup::SPACING,
};

pub fn camera_controls(
    camera_query: Single<(&mut Transform, &mut Projection), With<Camera>>,
    k_input: Res<ButtonInput<KeyCode>>,
    m_input: Res<ButtonInput<MouseButton>>,
    mut m_motion_reader: MessageReader<MouseMotion>,
    mut m_wheel_reader: MessageReader<MouseWheel>,
    time: Res<Time<Fixed>>,
) {
    let (mut transform, mut projection) = camera_query.into_inner();

    if let Projection::Orthographic(projection2d) = &mut *projection {
        // Camera movement control
        if m_input.pressed(MouseButton::Right) {
            for m_motion in m_motion_reader.read() {
                transform.translation.x -= m_motion.delta.x * projection2d.scale;
                transform.translation.y += m_motion.delta.y * projection2d.scale;
            }
        }

        // Camera zoom controls
        for m_wheel in m_wheel_reader.read() {
            if m_wheel.y < 0.0 {
                projection2d.scale *= powf(10.0, time.delta_secs());
            } else if m_wheel.y > 0.0 {
                projection2d.scale *= powf(0.1, time.delta_secs());
            }
        }

        // Zero camera
        if k_input.pressed(KeyCode::Space) {
            transform.translation = Vec3::ZERO;
            projection2d.scale = 1.0;
        }
    }
}

pub fn place_rail(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    m_input: Res<ButtonInput<MouseButton>>,
    mut railnodes_query: Query<&mut RailNode>,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
        && m_input.just_pressed(MouseButton::Left)
    {
        let snap_point = (world_pos / SPACING).round() * SPACING;

        for rail_node in railnodes_query.iter() {
            for node in rail_node.nodes.iter() {
                if snap_point == *node {
                    return;
                }
            }
        }

        let placed = PlacedRail::straight(snap_point, &asset_server);

        commands.spawn((
            placed.clone(),
            Transform::from_translation(placed.position.extend(0.0)),
            Sprite::from_image(placed.sprite),
            Name::new("Straight Rail"),
        ));

        let nd1 = placed.position + placed.path[0];
        let nd2 = placed.position + placed.path[1];
        let nd3 = placed.position + placed.path[2];
        let mut pushed = false;

        for mut rail_nodes in railnodes_query.iter_mut() {
            if *rail_nodes.nodes.last().unwrap() == nd1 {
                rail_nodes.nodes.push(nd2);
                rail_nodes.nodes.push(nd3);
                pushed = true;
            } else if *rail_nodes.nodes.last().unwrap() == nd3 {
                rail_nodes.nodes.push(nd2);
                rail_nodes.nodes.push(nd1);
                pushed = true;
            } else if *rail_nodes.nodes.first().unwrap() == nd1 {
                let mut updated_node = vec![nd3, nd2];
                updated_node.append(&mut rail_nodes.nodes);
                rail_nodes.nodes = updated_node;
                pushed = true;
            } else if *rail_nodes.nodes.first().unwrap() == nd3 {
                let mut updated_node = vec![nd2, nd1];
                updated_node.append(&mut rail_nodes.nodes);
                rail_nodes.nodes = updated_node;
                pushed = true;
            }
        }

        if railnodes_query.is_empty() || !pushed {
            commands.spawn((
                RailNode {
                    nodes: vec![nd1, nd2, nd3],
                },
                Name::new("Rail Node"),
            ));
        }
    }
}

pub fn remove_rail(
    mut commands: Commands,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    m_input: Res<ButtonInput<MouseButton>>,
    placed_rail_query: Query<(&PlacedRail, Entity)>,
    // mut railnodes_query: Query<&mut RailNode>,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
        && m_input.just_pressed(MouseButton::Right)
    {
        let snap_point = (world_pos / SPACING).round() * SPACING;

        for (placed_rail, entity) in placed_rail_query.iter() {
            if placed_rail.position == snap_point {
                commands.entity(entity).despawn();
            }
        }
    }
}
