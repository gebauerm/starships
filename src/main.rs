use bevy::input::keyboard::Key;
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

const SHIP_ROTATION_SPEED: f32 = f32::to_radians(5.0);
const SHIP_THRUST: f32 = 0.2;
const MAX_SHIP_VELOCITY: f32 = 10.;

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Default)]
#[require(Transform)]
struct Facing(Quat);

#[derive(Component, Default)]
#[require(Facing)]
struct Thrust(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
#[require(Position, Thrust = Thrust(Vec2::ZERO), Velocity = Velocity(Vec2::ZERO))]
struct Ship;

#[derive(Component)]
//#[require(Ship)]
struct Attacker;

#[derive(Component)]
//#[require(Ship)]
struct Defender;


enum ShipThrust {
    ON,
    OFF,
    BACK
}
enum ShipRotation {
    LEFT,
    RIGHT,
    IDLE
}


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
        Ship,
        Sprite::from_image(attacker_img.clone()),
        Position(attacker_pos),
        Facing(attacker_facing.clone()),
    ));

    commands.spawn((
        Defender,
        Ship,
        Sprite::from_image(attacker_img),
        Position(defender_pos),
        Facing(attacker_facing.inverse()),
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}


fn manage_movement_boundaries()
{

}


fn update_ship_velocity(keyboard_input: Res<ButtonInput<KeyCode>>,
    velocity_variables: Query<(&mut Velocity, &mut Thrust, &mut Facing), With<Ship>>) {

    let mut ship_thrust_input: f32 = 0.0;
    let mut ship_rotation_input: f32 = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        ship_thrust_input = 1.;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        ship_thrust_input = 0.;
    } else {
        ship_thrust_input = 0.;
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        ship_rotation_input = 1.;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        ship_rotation_input = -1.;
    }
    else {
        ship_rotation_input = 0.;
    }

    for (mut velocity, mut thrust, mut facing) in velocity_variables {
        let (axis, angle) = facing.0.to_axis_angle();
        let mut angle = angle * axis.z;
        angle += SHIP_ROTATION_SPEED * ship_rotation_input;
        facing.0 = Quat::from_rotation_z(angle);

        thrust.0 = Vec2::from_angle(angle).rotate(Vec2::Y) * SHIP_THRUST * ship_thrust_input;
        velocity.0 += thrust.0;
        if velocity.0.length_squared() > MAX_SHIP_VELOCITY.powi(2) {
            velocity.0 = velocity.0.normalize() * MAX_SHIP_VELOCITY;
        }
    }
}


fn update_positions(movement_variables: Query<(&mut Position, &Velocity)>)
{
    for (mut position, velocity) in movement_variables {
        position.0 += velocity.0;
    }
}



fn sprite_movement() {
    // https://docs.rs/bevy/0.18.1/src/move_sprite/move_sprite.rs.html#23
    // https://bevy.org/examples/2d-rendering/rotation/
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_players))
        .add_systems(FixedUpdate, (project_positions, update_ship_velocity.before(project_positions),
            update_positions.after(update_ship_velocity)))
        .run();
}

