//! The player ship: controls, spawning, and thrust physics.

use crate::{combat, config, movement, player_config, projectiles, timers};
use bevy::prelude::*;
use crate::player_config::PlayerConfig;

#[derive(Component, Default)]
#[require(movement::Position, movement::Thrust = movement::Thrust(Vec2::ZERO), movement::Velocity = movement::Velocity(Vec2::ZERO), combat::Health = combat::Health(config::SHIP_HEALTH),
combat::Collider = combat::Collider(Rectangle::new(config::SHIP_SIZE-10., config::SHIP_SIZE-10.)), timers::ShootDelayTimer= timers::ShootDelayTimer::default(), timers::BoostDelayTimer = timers::BoostDelayTimer::default(), timers::BoostDurationTimer=timers::BoostDurationTimer::default())]
pub struct Ship;

#[derive(Component, Default)]
pub struct PlayerColor(pub Color);

#[derive(Resource)]
pub struct Shipsprite(Sprite);

fn load_ship_sprite(asset_server: &AssetServer) -> Shipsprite {
    let ship_img = asset_server.load("player.png");
    let mut sprite = Sprite::from_image(ship_img.clone());
    sprite.custom_size = Some(Vec2::new(config::SHIP_SIZE, config::SHIP_SIZE));
    Shipsprite(sprite)
}

pub fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(load_ship_sprite(&asset_server));
}

#[derive(Component)]
#[require(Ship)]
pub struct PlayerControls {
    acc: KeyCode,
    reverse: KeyCode,
    left: KeyCode,
    right: KeyCode,
    fire: KeyCode,
    boost: KeyCode,
    thrust_input: f32,
    rotation_input: f32,
}

impl Default for PlayerControls {
    fn default() -> Self {
        Self {
            acc: KeyCode::KeyW,
            reverse: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            fire: KeyCode::Space,
            boost: KeyCode::ShiftLeft,
            thrust_input: 0.,
            rotation_input: 0.,
        }
    }
}

impl From<&PlayerConfig> for PlayerControls {
    /// Uses the From trait to implement a constructor. The trait consumes the value an uses it for construction.
    /// Also implements "into" under the hood
    /// For more: https://doc.rust-lang.org/std/convert/trait.From.html
    fn from(cfg: &PlayerConfig) -> Self {
        Self {
            acc: cfg.acc,
            reverse: cfg.reverse,
            left: cfg.left,
            right: cfg.right,
            fire: cfg.fire,
            boost: cfg.boost,
            thrust_input: 0.,
            rotation_input: 0.,
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    // TODO: move this into player_config, together with controls. Players should be spawne directly from player_config.
    controls: PlayerControls,
    sprite: Sprite,
    position: movement::Position,
    facing: movement::Facing,
    health: combat::Health,
    shoot_delay_timer: timers::ShootDelayTimer,
    ship: Ship,
    color: PlayerColor,
}

impl PlayerBundle {
    fn new(
        player_config: &player_config::PlayerConfig,
        sprite: Sprite,
        window: &Single<&Window>,
    ) -> Self {
        let (position, facing) = player_config.starting_positions(window);
        let sprite = player_config.color_sprites(sprite);
        let controls = PlayerControls::from(player_config);
        Self {
            controls,
            sprite,
            position: movement::Position(position),
            facing: movement::Facing(facing),
            health: combat::Health(config::SHIP_HEALTH),
            shoot_delay_timer: timers::ShootDelayTimer::default(),
            ship: Ship,
            color: PlayerColor(player_config.color),
        }
    }
}

pub fn handle_player_inputs(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    players: Query<(&mut PlayerControls, Entity), With<Ship>>,
) {
    for (mut player, entity) in players {
        if keyboard_input.pressed(player.acc) {
            player.thrust_input = 1.;
        } else if keyboard_input.pressed(player.reverse) {
            player.thrust_input = -1.;
        } else if keyboard_input.just_pressed(player.boost) {
            // here implement boost
        }

        if keyboard_input.pressed(player.left) {
            player.rotation_input = 1.;
        } else if keyboard_input.pressed(player.right) {
            player.rotation_input = -1.;
        } else {
            player.rotation_input = 0.;
        }

        if keyboard_input.pressed(player.fire) {
            commands.trigger(projectiles::Shoot { shooter: entity });
        }
    }
}

pub fn update_ship_velocity(
    variables: Query<
        (
            &mut movement::Velocity,
            &mut movement::Thrust,
            &mut movement::Facing,
            &PlayerControls,
        ),
        With<Ship>,
    >,
) {
    for (mut velocity, mut thrust, mut facing, player) in variables {
        let (axis, angle) = facing.0.to_axis_angle();
        let mut angle = angle * axis.z;
        angle += config::SHIP_ROTATION_SPEED * player.rotation_input;
        facing.0 = Quat::from_rotation_z(angle);

        thrust.0 =
            facing.direction() * config::SHIP_THRUST * player.thrust_input;
        if player.thrust_input < 0. {
            thrust.0 = velocity.0.normalize() * config::SHIP_BREAKS * player.thrust_input;
        }
        velocity.0 += thrust.0;
        if velocity.0.length_squared() > config::MAX_SHIP_VELOCITY.powi(2) {
            velocity.0 = velocity.0.normalize() * config::MAX_SHIP_VELOCITY;
        }
    }
}

fn spawn_player<C: Component>(
    player_config: &player_config::PlayerConfig,
    window: &Single<&Window>,
    sprite: Sprite,
    role_marker: C,
) -> (PlayerBundle, impl Component) {
    let player_bundle = PlayerBundle::new(player_config, sprite, window);

    (player_bundle, role_marker)
}

pub fn spawn_players(
    mut commands: Commands,
    window: Single<&Window>,
    ship_sprite: Res<Shipsprite>,
) {
    for player_config in player_config::PLAYER_CONFIGS.iter() {
        match player_config.role {
            player_config::PlayerRole::Attacker => {
                let (player_bundle, role) = spawn_player(
                    player_config,
                    &window,
                    ship_sprite.0.clone(),
                    player_config::Attacker,
                );
                commands.spawn((player_bundle, role));
            }
            player_config::PlayerRole::Defender => {
                let (player_bundle, role) = spawn_player(
                    player_config,
                    &window,
                    ship_sprite.0.clone(),
                    player_config::Defender,
                );
                commands.spawn((player_bundle, role));
            }
        }
    }
}
