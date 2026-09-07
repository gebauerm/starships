//! Cooldown timers shared between ship and projectile systems

use bevy::prelude::*;
use crate::config;
use crate::PlayerControls;


#[derive(Component)]
pub struct ShootDelayTimer {
    pub timer: Timer,
}
impl ShootDelayTimer {
    pub fn default() -> Self {
        Self {
            timer: Timer::from_seconds(config::MAX_SHOT_DELAY, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct BoostDelayTimer {
    pub timer: Timer,
}
impl BoostDelayTimer {
    pub fn default() -> Self {
        let mut timer = Timer::from_seconds(config::BOOST_DELAY, TimerMode::Once);
        // timer.tick(Duration::from_secs_f32(BOOST_DELAY + 1.));
        timer.finish();
        Self { timer }
    }
}


#[derive(Component)]
pub struct BoostDurationTimer {
    pub timer: Timer,
}
impl BoostDurationTimer {
    pub fn default() -> Self {
        Self {
            timer: Timer::from_seconds(config::BOOST_DURATION, TimerMode::Once),
        }
    }
}


pub fn tick_timers(
    timers: Query<&mut ShootDelayTimer, With<PlayerControls>>,
    boost_duration: Query<&mut BoostDurationTimer, With<PlayerControls>>,
    boost_delay: Query<&mut BoostDelayTimer, With<PlayerControls>>,
    time: Res<Time>,
) {
    //TODO: this needs to be wrapped in a nice pattern
    for mut timer in timers {
        timer.timer.tick(time.delta());
    }

    for mut timer in boost_duration {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            timer.timer.reset();
        }
    }

    for mut timer in boost_delay {
        timer.timer.tick(time.delta());
    }
}
