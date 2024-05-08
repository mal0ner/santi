mod camera;
mod gravity;
mod inspector;
mod line;

use crate::{camera::*, gravity::*, inspector::*, line::*};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    geometry::GeometryBuilder,
    plugin::ShapePlugin,
    shapes,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, move_circle)
        .add_systems(Update, update_vel)
        .add_systems(Update, update_line)
        .add_systems(Update, inspector_ui)
        .add_systems(Update, update_camera_center)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shape = shapes::Circle {
        radius: 32.0,
        center: Vec2 { x: 0.0, y: 0.0 },
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(-200., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        },
        Stroke::new(Color::WHITE, 10.0),
        Fill::color(Color::WHITE),
        Body,
        Velocity(Vec2::new(0.0, 0.2)),
        Mass(500.0),
    ));

    let earth = create_circle_mesh(30.0, 32);
    let saturn = create_circle_mesh(42.0, 32);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(saturn).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.8, 0.1, 0.3))),
            transform: Transform::from_xyz(0.0, 200.0, 0.0),
            ..Default::default()
        },
        Body,
        Velocity(Vec2::new(0.0, 0.2)),
        Mass(500.0),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(earth).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(0.1, 0.8, 0.3))),
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            ..Default::default()
        },
        Body,
        Velocity(Vec2::new(0.0, 18.0)),
        Mass(500.),
    ));
}
