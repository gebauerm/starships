//! Combat: hit points, collliers and collision detection

use bevy::prelude::*;
use crate::{ PlayerControls, config };
use bevy::math::bounding::{ Aabb2d, IntersectsVolume };
use crate::movement;
use crate::projectiles::Shot;

#[derive(Component, Default)]
pub struct Health(pub f32);

impl Health {
    pub fn is_zero(&self) -> bool {
        self.0 <= 0.0
    }
}

/// Collision Enum to determine the irection of the collision.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    FRONT,
    BACK,
    SIDE,
}

#[derive(Component, Default)]
pub struct Collider(pub Rectangle);
impl Collider {
    pub fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}

fn collision_with_shot(
    player: Aabb2d,
    player_facing: &movement::Facing,
    shot: Aabb2d,
    shot_facing: &movement::Facing
) -> Option<Collision> {
    if !player.intersects(&shot) {
        return None;
    }

    let player_dir = movement::direction_from_angle(player_facing.to_angle());
    let shot_dir = movement::direction_from_angle(shot_facing.to_angle());
    let hit_direction = player_dir.dot(shot_dir);

    if hit_direction >= config::BACK_HIT_THRESHOLD {
        Some(Collision::BACK)
    } else if hit_direction <= config::FRONT_HIT_THRESHOLD {
        Some(Collision::FRONT)
    } else {
        Some(Collision::SIDE)
    }
}

/// Detects the direction the ship is hit from and whether it is hit at all, by using vector calculations.
pub fn handle_shot_hits(
    mut commands: Commands,
    player_variables: Query<
        (&movement::Position, &movement::Facing, &Collider, &mut Health),
        With<PlayerControls>
    >,
    shot_variables: Query<(&movement::Position, &movement::Facing, &Collider, Entity), With<Shot>>
) {
    for (player_position, player_facing, player_collider, mut health) in player_variables {
        for (shot_position, shot_facing, shot_collider, shot) in shot_variables {
            if
                let Some(collision) = collision_with_shot(
                    Aabb2d::new(player_position.0, player_collider.half_size()),
                    player_facing,
                    Aabb2d::new(shot_position.0, shot_collider.half_size()),
                    shot_facing
                )
            {
                match collision {
                    Collision::FRONT => {
                        health.0 -= config::FRONT_SHOT_DMG;
                    }
                    Collision::BACK => {
                        health.0 -= config::BACK_SHOT_DMG;
                    }
                    Collision::SIDE => {
                        health.0 -= config::SIDE_SHOT_DMG;
                    }
                }
                commands.entity(shot).despawn();
            }
        }
    }
}

pub fn clear_dead_stuff(mut commands: Commands, entity_variables: Query<(Entity, &Health)>) {
    for (entity, health) in entity_variables {
        if health.is_zero() {
            commands.entity(entity).despawn();
        }
    }
}
