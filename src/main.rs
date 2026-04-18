use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

const SHIP_ROTATION_SPEED: f32 = f32::to_radians(5.0);
const SHIP_THRUST: f32 = 0.2;
const SHIP_HEALTH: f32 = 100.;
const SHIP_SIZE: f32 = 8.;
const MAX_SHIP_VELOCITY: f32 = 4.;

const MAX_SHOT_DELAY: f32 = 1.;
const MAX_SHOT_VELOCITY: f32 = 12.;

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Default)]
#[require(Transform)]
struct Facing(Quat);

#[derive(Component, Default)]
struct ShipHealth(f32);

#[derive(Component, Default)]
#[require(Facing)]
struct Thrust(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

impl Velocity {
    // This is used to initialize shots
    fn from_facing(facing: &Facing) -> Self {
        let (axis, angle) = facing.0.to_axis_angle();
        let angle = angle * axis.z;
        let thrust = Vec2::from_angle(angle).rotate(Vec2::Y).normalize() * MAX_SHOT_VELOCITY;
        Self(thrust)
    }
}

#[derive(Component, Default)]
#[require(Position, Thrust = Thrust(Vec2::ZERO), Velocity = Velocity(Vec2::ZERO))]
struct Shot;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    FRONT,
    BACK,
    SIDE,
}

#[derive(Component, Default)]
struct Collider(Rectangle);
impl Collider {
    fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}

#[derive(Component, Default)]
#[require(Position, Thrust = Thrust(Vec2::ZERO), Velocity = Velocity(Vec2::ZERO), ShipHealth = ShipHealth(SHIP_HEALTH),
Collider = Collider(Rectangle::new(SHIP_SIZE, SHIP_SIZE)))]
struct Ship;

#[derive(Component, Default)]
struct PlayerColor(Color);

#[derive(Component)]
#[require(Ship)]
struct Player {
    acc: KeyCode,
    reverse: KeyCode,
    left: KeyCode,
    right: KeyCode,
    fire: KeyCode,
    thrust_input: f32,
    rotation_input: f32,
}
impl Player {
    fn new(acc: KeyCode, reverse: KeyCode, left: KeyCode, right: KeyCode, fire: KeyCode) -> Self {
        Self {
            acc,
            reverse,
            left,
            right,
            fire,
            thrust_input: 0.,
            rotation_input: 0.,
        }
    }
}

#[derive(Component)]
#[require(Player= Player::new(
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
            KeyCode::Numpad0,
        ), PlayerColor(Color::srgb(1., 0.5, 0.)))]
struct Attacker;

#[derive(Component)]
#[require(Player = Player::new(
            KeyCode::KeyW,
            KeyCode::KeyS,
            KeyCode::KeyA,
            KeyCode::KeyD,
            KeyCode::Space
        ), PlayerColor(Color::srgb(0.1, 0.7, 1.)))]
struct Defender;

#[derive(EntityEvent)]
struct Shoot {
    #[event_target]
    shooter: Entity,
}

#[derive(Component)]
struct ShootDelayTimer {
    timer: Timer,
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position, &Facing)>) {
    for (mut transform, position, facing) in &mut positionables {
        // Extend is going to turn this from a Vec2 to a Vec3
        transform.translation = position.0.extend(0.);
        transform.rotation = facing.0;
    }
}

fn tick_timers(timers: Query<&mut ShootDelayTimer, With<Player>>, time: Res<Time>) {
    for mut timer in timers {
        timer.timer.tick(time.delta());
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

    // TODO: color setting needs refactoring (keep the coloring of the shots in mind)
    let attacker_color = PlayerColor(Color::srgb(1., 0.5, 0.));
    let defender_color = PlayerColor(Color::srgb(0., 0.5, 1.));
    let mut attacker_sprite = Sprite::from_image(attacker_img.clone());
    attacker_sprite.color = attacker_color.0;
    let mut defender_sprite = Sprite::from_image(attacker_img);
    defender_sprite.color = defender_color.0;

    commands.spawn((
        Attacker,
        Ship,
        attacker_sprite,
        Position(attacker_pos),
        Facing(attacker_facing.clone()),
        ShootDelayTimer {
            timer: Timer::from_seconds(MAX_SHOT_DELAY, TimerMode::Once),
        },
        ShipHealth(SHIP_HEALTH),
    ));

    commands.spawn((
        Defender,
        Ship,
        defender_sprite,
        Position(defender_pos),
        Facing(attacker_facing.inverse()),
        ShootDelayTimer {
            timer: Timer::from_seconds(MAX_SHOT_DELAY, TimerMode::Once),
        },
        ShipHealth(SHIP_HEALTH),
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}

fn enforce_movement_limits(
    window: Single<&Window>,
    ship_positions: Query<&mut Position, With<Ship>>,
) {
    let max_window_height = window.resolution.height() / 2.;
    let max_window_width = window.resolution.width() / 2.;

    for mut ship_position in ship_positions {
        if ship_position.0.x > max_window_width {
            ship_position.0.x -= window.resolution.width();
        } else if ship_position.0.x < -max_window_width {
            ship_position.0.x += window.resolution.width();
        }

        if ship_position.0.y > max_window_height {
            ship_position.0.y -= window.resolution.height();
        } else if ship_position.0.y < -max_window_height {
            ship_position.0.y += window.resolution.height();
        }
    }
}

fn handle_player_inputs(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    players: Query<(&mut Player, Entity), With<Ship>>,
) {
    for (mut player, entity) in players {
        if keyboard_input.pressed(player.acc) {
            player.thrust_input = 1.;
        } else if keyboard_input.pressed(player.reverse) {
            player.thrust_input = 0.;
        } else {
            player.thrust_input = 0.;
        }

        if keyboard_input.pressed(player.left) {
            player.rotation_input = 1.;
        } else if keyboard_input.pressed(player.right) {
            player.rotation_input = -1.;
        } else {
            player.rotation_input = 0.;
        }

        if keyboard_input.pressed(player.fire) {
            commands.trigger(Shoot { shooter: entity });
        }
    }
}

fn update_ship_velocity(
    variables: Query<(&mut Velocity, &mut Thrust, &mut Facing, &Player), With<Ship>>,
) {
    for (mut velocity, mut thrust, mut facing, player) in variables {
        let (axis, angle) = facing.0.to_axis_angle();
        let mut angle = angle * axis.z;
        angle += SHIP_ROTATION_SPEED * player.rotation_input;
        facing.0 = Quat::from_rotation_z(angle);

        thrust.0 = Vec2::from_angle(angle).rotate(Vec2::Y) * SHIP_THRUST * player.thrust_input;
        velocity.0 += thrust.0;
        if velocity.0.length_squared() > MAX_SHIP_VELOCITY.powi(2) {
            velocity.0 = velocity.0.normalize() * MAX_SHIP_VELOCITY;
        }
    }
}

fn update_positions(movement_variables: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in movement_variables {
        position.0 += velocity.0;
    }
}

fn spawn_shots(
    event: On<Shoot>,
    mut commands: Commands,
    mut variables: Query<(&Position, &Facing, &mut ShootDelayTimer, &PlayerColor), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let attacker_img = asset_server.load("shot.png");
    let mut sprite = Sprite::from_image(attacker_img);
    if let Ok((position, facing, mut config, player_color)) = variables.get_mut(event.shooter) {
        if config.timer.is_finished() {
            sprite.color = player_color.0;
            commands.spawn((
                Shot,
                Transform::from_translation(position.0.extend(0.)),
                Position(position.0.clone()),
                Velocity::from_facing(facing),
                sprite,
            ));
            config.timer.reset();
        }
    }
}

fn handle_shot_hits() {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_players))
        .add_systems(
            FixedUpdate,
            (
                project_positions,
                tick_timers.before(handle_player_inputs),
                handle_player_inputs.before(update_ship_velocity),
                update_ship_velocity.before(project_positions),
                update_positions.after(update_ship_velocity),
                enforce_movement_limits.after(update_positions),
            ),
        )
        .add_observer(spawn_shots)
        .run();
}
