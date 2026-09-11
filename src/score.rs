//! Scoring and match reset
//! Currently disabled

use crate::combat::Health;
use crate::player_config;
use crate::ship;
use crate::ship::Shipsprite;
use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct Scored {
    entity: Entity,
}

#[derive(Resource)]
pub struct Score {
    pub attacker: u8,
    pub defender: u8,
}

fn detect_player_destruction(
    attacker: Single<(Entity, &Health), With<player_config::Attacker>>,
    defender: Single<(Entity, &Health), With<player_config::Defender>>,
) {
    let (attacker, attacker_health) = attacker.into_inner();
    let (defender, defender_health) = defender.into_inner();

    if defender_health.is_zero() {
        Scored { entity: attacker };
    }

    if attacker_health.is_zero() {
        Scored { entity: defender };
    }
}

pub fn update_score(
    event: On<Scored>,
    mut score: ResMut<Score>,
    attacker: Query<Entity, With<player_config::Attacker>>,
    defender: Query<Entity, With<player_config::Defender>>,
) {
    if attacker.get(event.entity).is_ok() {
        score.attacker += 1;
    }
    if defender.get(event.entity).is_ok() {
        score.defender += 1;
    }
}

pub fn reset_game(
    _event: On<Scored>,
    window: Single<&Window>,
    commands: Commands,
    ship_sprite: Res<Shipsprite>,
) {
    ship::spawn_players(commands, window, ship_sprite);
}
