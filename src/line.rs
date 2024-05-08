use crate::gravity::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
pub struct Line;

pub fn update_line(
    mut commands: Commands,
    mut query: Query<&Transform, With<Body>>,
    line_query: Query<(Entity, &mut Path), With<Line>>,
) {
    let mut lines: Vec<Entity> = Vec::new();
    let mut combinations = query.iter_combinations_mut::<2>();
    while let Some([t1, t2]) = combinations.fetch_next() {
        let line_entity = commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Line(
                    Vec2::new(t1.translation.x, t1.translation.y),
                    Vec2::new(t2.translation.x, t2.translation.y),
                )),
                // faux z-index, explicitly place lines below
                // bodies.
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0., 0., -1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Stroke::new(Color::WHITE, 0.3),
            Fill::color(Color::WHITE),
            Line,
        ));
        lines.push(line_entity.id())
    }

    // clean up old lines
    for (existing_line, _) in line_query.iter() {
        commands.entity(existing_line).despawn();
    }
}
