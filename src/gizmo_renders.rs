use bevy::{
    color::palettes::{
        basic::WHITE,
        css::{MAGENTA, YELLOW},
    },
    prelude::*,
};

use crate::{
    rail_types::{PlacedRail, RailNode},
    setup::SPACING,
};

pub fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        gizmos.circle_2d(world_pos, 5., WHITE);

        let snap_point = Vec2::new(
            (world_pos.x / SPACING).round() * SPACING,
            (world_pos.y / SPACING).round() * SPACING,
        );
        gizmos.circle_2d(snap_point, 5., YELLOW);
    }
}

pub fn draw_grid(mut gizmos: Gizmos, camera_q: Single<(&Transform, &mut Projection)>) {
    let (cam_transform, mut projection) = camera_q.into_inner();

    if let Projection::Orthographic(proj) = &mut *projection {
        let view_w = proj.area.width();
        let view_h = proj.area.height();
        let cam_pos = cam_transform.translation.truncate();

        let start_x = ((cam_pos.x - view_w / 2.0) / SPACING).floor() * SPACING;
        let end_x = cam_pos.x + view_w / 2.0;
        let mut x = start_x;
        while x <= end_x {
            gizmos.line_2d(
                Vec2::new(x, cam_pos.y - view_h),
                Vec2::new(x, cam_pos.y + view_h),
                Color::srgba(0.5, 0.5, 0.5, 0.15),
            );

            x += SPACING;
        }

        let start_y = ((cam_pos.y - view_h / 2.0) / SPACING).floor() * SPACING;
        let end_y = cam_pos.y + view_h / 2.0;
        let mut y = start_y;
        while y <= end_y {
            gizmos.line_2d(
                Vec2::new(cam_pos.x - view_w, y),
                Vec2::new(cam_pos.x + view_w, y),
                Color::srgba(0.5, 0.5, 0.5, 0.15),
            );
            y += SPACING;
        }
    }
}

pub fn draw_ports(rail_query: Query<&PlacedRail>, mut gizmos: Gizmos) {
    for rail in rail_query.into_iter() {
        for port in rail.ports.iter() {
            let port_pos = Vec2::new(
                rail.position.x + port.offset.x,
                rail.position.y + port.offset.y,
            );

            let rad = port.facing_deg.to_radians();
            let end_pos = port_pos + 15.0 * Vec2::new(rad.sin(), rad.cos());

            gizmos.circle_2d(port_pos, 5., WHITE);
            gizmos.line_2d(port_pos, end_pos, YELLOW);
        }
    }
}

pub fn draw_nodes(railnodes_query: Query<&RailNode>, mut gizmos: Gizmos) {
    for rail_nodes in railnodes_query.iter() {
        for (i, node) in rail_nodes.nodes.iter().enumerate() {
            if i < rail_nodes.nodes.len() - 1 {
                gizmos.line_2d(*node, rail_nodes.nodes[i + 1], MAGENTA);
            }
        }
    }
}
