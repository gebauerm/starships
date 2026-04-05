use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;

#[derive(Component, Default)]
struct Position(Vec2);

#[derive(Component)]
#[require(Position)]
struct Attacker;

#[derive(Component)]
#[require(Position)]
struct Defender;

fn spawn_players(mut commands: Commands, window: Single<&Window>) {
    let half_window_size = window.resolution.size() / 2.;
    let padding = 20.;

    let attacker_pos = Vec2::new(half_window_size.x - padding, 0.);
    commands.spawn((Attacker, Position(attacker_pos)));

    let defender_pos = Vec2::new(-half_window_size.x + padding, 0.);
    commands.spawn((Defender, Position(defender_pos)));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_players))
        .run();
}
