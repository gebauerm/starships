//! Projectiles: the Shot entity and how shots spawn.

use bevy::prelude::*;

use crate::combat;
use crate::config;
use crate::movement::{Facing, Position, Velocity, direction_from_angle};
use crate::ship::{PlayerColor, PlayerControls};
use crate::sprites;
use crate::timers::ShipCooldowns;

#[derive(Component, Default)]
#[require(
    crate::movement::Position,
    crate::movement::Facing,
    crate::movement::Thrust = crate::movement::Thrust(Vec2::ZERO),
    crate::movement::Velocity = crate::movement::Velocity(Vec2::ZERO),
    combat::Collider = combat::Collider(Rectangle::new(config::SHOT_SIZE, config::SHOT_SIZE)),
    combat::Health = combat::Health(1.0)
)]
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
        (&Position, &Facing, &mut ShipCooldowns, &PlayerColor),
        With<PlayerControls>,
    >,
    sprite: Res<sprites::ShotSprite>,
) {
    let mut sprite = sprite.0.clone();

    if let Ok((position, facing, mut ship_cooldowns, player_color)) =
        variables.get_mut(event.shooter)
    {
        if ship_cooldowns.shot.is_finished() {
            sprite.color = player_color.0;
            let position = position.0
                + direction_from_angle(facing.to_angle())
                    * (config::SHIP_SIZE / 2.0 + config::SHOOT_OFFSET);
            commands.spawn((
                Shot,
                Facing(facing.0),
                Transform::from_translation(position.extend(0.0)),
                Position(position),
                Velocity::from_facing(facing),
                sprite,
            ));
            ship_cooldowns.shot.reset();
        }
    }
}
