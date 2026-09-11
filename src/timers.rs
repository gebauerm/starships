//! Cooldown timers shared between ship and projectile systems
use crate::PlayerControls;
use crate::config;
use bevy::prelude::*;

#[derive(Component)]
pub struct ShipCooldowns {
    pub shot: Timer,
}

impl Default for ShipCooldowns {
    fn default() -> Self {
        Self {
            shot: Timer::from_seconds(config::SHOT_DELAY, TimerMode::Once),
        }
    }
}

pub fn tick_timers(
    ship_cooldowns: Query<&mut ShipCooldowns, With<PlayerControls>>,
    time: Res<Time>,
) {
    //TODO: this needs to be wrapped in a nice pattern
    for mut ship_cooldown in ship_cooldowns {
        ship_cooldown.shot.tick(time.delta());
    }
}
