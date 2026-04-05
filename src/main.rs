use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Default)]
#[require(Transform)]
struct Facing(Quat);

#[derive(Component)]
#[require(Position, Facing)]
struct Attacker;

#[derive(Component)]
#[require(Position, Facing)]
struct Defender;

fn project_positions(mut positionables: Query<(&mut Transform, &Position, &Facing)>) {
    for (mut transform, position, facing) in &mut positionables {
        // Extend is going to turn this from a Vec2 to a Vec3
        transform.translation = position.0.extend(0.);
        transform.rotation = facing.0;
    }
}
fn spawn_players(mut commands: Commands, window: Single<&Window>, asset_server: Res<AssetServer>) {
    let half_window_size = window.resolution.size() / 2.;
    let padding = 20.;

    let attacker_img = asset_server.load("player.png");

    let attacker_pos = Vec2::new(half_window_size.x - padding, 0.);
    let defender_pos = Vec2::new(-half_window_size.x + padding, 0.);
    let attacker_facing =
        Quat::from_rotation_z((defender_pos - attacker_pos).to_angle() - FRAC_PI_2);

    commands.spawn((
        Attacker,
        Sprite::from_image(attacker_img.clone()),
        Position(attacker_pos),
        Facing(attacker_facing.clone()),
    ));

    commands.spawn((
        Defender,
        Sprite::from_image(attacker_img),
        Position(defender_pos),
        Facing(attacker_facing.inverse()),
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}

fn sprite_movement() {
    // https://docs.rs/bevy/0.18.1/src/move_sprite/move_sprite.rs.html#23
    // https://bevy.org/examples/2d-rendering/rotation/
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_players))
        .add_systems(FixedUpdate, project_positions)
        .run();
}
