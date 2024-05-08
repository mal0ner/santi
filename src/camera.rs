use crate::gravity::*;
use bevy::{math::f32, prelude::*, window::PrimaryWindow};

const MIN_PADDING: f32 = 100.0;
const SCALE_ADJ_SMOOTH_FAC: f32 = 0.01;

pub fn update_camera_center(
    mut camera_query: Query<(&Camera, &mut Transform, &mut OrthographicProjection), With<Camera>>,
    body_query: Query<&Transform, (With<Body>, Without<Camera>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let win_size = Vec2::new(window.width(), window.height());

    for (camera, mut cam_transform, mut projection) in camera_query.iter_mut() {
        let mut acc_pos = Vec3::ZERO;
        let mut min_dist = f32::MAX;

        for body_transform in body_query.iter() {
            let pos = body_transform.translation;
            acc_pos += pos;

            let g_transform = GlobalTransform::from(*cam_transform);
            if let Some(ndc) = camera.world_to_ndc(&g_transform, pos) {
                let screen_pos = (ndc + Vec3::ONE) / 2.0 * Vec3::new(win_size.x, win_size.y, 1.0);
                let edge_dists = [
                    screen_pos.x,
                    win_size.x - screen_pos.x,
                    win_size.y - screen_pos.y,
                    screen_pos.y,
                ];
                min_dist = edge_dists.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            } else {
                info!("could not find projection scaled distance");
            }
        }

        cam_transform.translation = acc_pos / body_query.iter().count() as f32;

        if min_dist < MIN_PADDING * projection.scale {
            let target_scale = calc_scale(min_dist, projection.scale);
            projection.scale = smooth_scale(projection.scale, target_scale, SCALE_ADJ_SMOOTH_FAC)
        }
    }
}

fn calc_scale(min_dist: f32, current_scale: f32) -> f32 {
    current_scale * (MIN_PADDING / min_dist)
}

fn smooth_scale(a: f32, b: f32, smooth_fac: f32) -> f32 {
    a + (b - a) * smooth_fac
}
