//! Combat: hit points, collliers and collision detection

use bevy::prelude::*;
use crate::{PlayerControls, Shot, config};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use crate::movement;

#[derive(Component, Default)]
pub struct Health(pub f32);

impl Health {
    pub fn is_dead(&self) -> bool {
        self.0 <= 0.
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    FRONT,
    BACK,
    SIDE
}

#[derive(Component, Default)]
pub struct Collider(pub Rectangle);
impl Collider {
    pub fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}


fn collision_with_shot(player: Aabb2d, shot: Aabb2d) -> Option<Collision> {
    if !player.intersects(&shot) {
        return None;
    }
    Some(Collision::FRONT)
}

pub fn handle_shot_hits(
    mut commands: Commands,
    player_variables: Query<(&movement::Position, &Collider, &mut Health), With<PlayerControls>>,
    shot_variables: Query<(&movement::Position, &Collider, Entity), With<Shot>>,
) {
    for (player_position, player_collider, mut health) in player_variables {
        for (shot_position, shot_collider, shot) in shot_variables {
            if let Some(collision) = collision_with_shot(
                Aabb2d::new(player_position.0, player_collider.half_size()),
                Aabb2d::new(shot_position.0, shot_collider.half_size()),
            ) {
                match collision {
                    Collision::FRONT => {
                        health.0 -= config::SHOT_DMG;
                    }
                    Collision::BACK => {
                        health.0 -= config::SHOT_DMG;
                    }
                    Collision::SIDE => {
                        health.0 -= config::SHOT_DMG;
                    }
                }
                commands.entity(shot).despawn();
            }
        }
    }
}


pub fn clear_dead_stuff(mut commands: Commands, entity_variables: Query<(Entity, &Health)>) {
    for (entity, health) in entity_variables {
        if health.0 <= 0. {
            commands.entity(entity).despawn();
        }
    }
}