use bevy::prelude::*;
use bevy::render::mesh::CircleMeshBuilder;

#[derive(Component)]
pub struct Body;

#[derive(Component, Debug, Copy, Clone)]
pub struct Position(pub Vec2);

#[derive(Component, Copy, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Component, Copy, Clone)]
pub struct Mass(pub f32);

pub fn create_circle_mesh(radius: f32, vert_count: usize) -> Mesh {
    CircleMeshBuilder {
        circle: Circle { radius: (radius) },
        resolution: vert_count,
    }
    .build()
}

pub fn move_circle(mut query: Query<(&mut Transform, &Velocity), With<Body>>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        let translation = velocity.0 * time.delta_seconds();
        transform.translation += translation.extend(0.0);
    }
}

pub fn update_vel(mut query: Query<(&mut Velocity, &mut Transform, &Mass), With<Body>>) {
    let mut combinations = query.iter_combinations_mut();
    let soft_fac: f32 = 0.0;
    while let Some([(mut vel1, transform1, mass1), (mut vel2, transform2, mass2)]) =
        combinations.fetch_next()
    {
        // simulates larger distances than can feasibly fit on screen
        let dist_scale_fac = 40.0;
        let distance = transform1.translation.distance(transform2.translation);
        // info!("Distance between objects: {}", distance);
        let dist_sqr_scaled = (distance * dist_scale_fac).powi(2);

        // Update velocities based on the distance between objects
        // You can implement your physics logic here
        // For example, you can apply gravitational forces or collision responses

        // Example: Simple repulsion force
        let direction = (transform2.translation - transform1.translation).normalize();
        let force = mass1.0 * mass2.0 / (dist_sqr_scaled) + (soft_fac.powi(2));
        vel1.0 += Vec2 {
            x: direction.x,
            y: direction.y,
        } * force;
        vel2.0 -= Vec2 {
            x: direction.x,
            y: direction.y,
        } * force;
    }
}
