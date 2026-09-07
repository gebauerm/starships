//! Projectiles: the Shot entity and how shots spawn.

use bevy::prelude::*;

use crate::movement::{Velocity, Position, Facing, direction_from_angle};
use crate::ship::{PlayerColor, PlayerControls};
use crate::config;
use crate::combat;
use crate::timers::ShootDelayTimer;


#[derive(Component, Default)]
#[require(crate::movement::Position, crate::movement::Thrust = crate::movement::Thrust(Vec2::ZERO), 
crate::movement::Velocity = crate::movement::Velocity(Vec2::ZERO), 
combat::Collider=combat::Collider(Rectangle::new(config::SHOT_SIZE, config::SHOT_SIZE)), combat::Health = combat::Health(1.))]
pub struct Shot;



#[derive(EntityEvent)]
pub struct Shoot {
    #[event_target]
    pub shooter: Entity,
}


pub fn spawn_shots(
    event: On<Shoot>,
    mut commands: Commands,
    mut variables: Query<
        (&Position, &Facing, &mut ShootDelayTimer, &PlayerColor),
        With<PlayerControls>,
    >,
    asset_server: Res<AssetServer>,
) {
    let attacker_img = asset_server.load("shot.png");
    let mut sprite = Sprite::from_image(attacker_img);
    sprite.custom_size = Some(Vec2::new(config::SHOT_SIZE, config::SHOT_SIZE));

    if let Ok((position, facing, mut shoot_delay, player_color)) =
        variables.get_mut(event.shooter)
    {
        if shoot_delay.timer.is_finished() {
            sprite.color = player_color.0;
            let position = position.0
                + direction_from_angle(facing.to_angle())
                    * (config::SHIP_SIZE / 2. + config::SHOOT_OFFSET);
            commands.spawn((
                Shot,
                Transform::from_translation(position.extend(0.)),
                Position(position),
                Velocity::from_facing(facing),
                sprite,
            ));
            shoot_delay.timer.reset();
        }
    }
}
