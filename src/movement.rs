//! Facing is store as a rotation quaternion "to_angle()" gives the signed Z rotation in radians

use crate::{Ship, config};
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(pub Vec2);

#[derive(Component, Default)]
#[require(Transform)]
pub struct Facing(pub Quat);
impl Facing {
    pub fn to_angle(&self) -> f32 {
        // given in radiants
        let (axis, angle) = self.0.to_axis_angle();
        let angle = angle * axis.z;
        angle
    }
    /// Unit vector in the direction the ship points.
    pub fn direction(&self) -> Vec2 {
        direction_from_angle(self.to_angle())
    }
}

#[derive(Component, Default)]
#[require(Facing)]
pub struct Thrust(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec2);

impl Velocity {
    /// This is used to initialize shots
    pub fn from_facing(facing: &Facing) -> Self {
        let thrust = facing.direction()* config::MAX_SHOT_VELOCITY;
        Self(thrust)
    }
}

/// Create a unit vector pointing along the given angle (raian)
pub fn direction_from_angle(angle: f32) -> Vec2 {
    Vec2::from_angle(angle).rotate(Vec2::Y).normalize()
}

pub fn project_positions(mut positionables: Query<(&mut Transform, &Position, &Facing)>) {
    for (mut transform, position, facing) in &mut positionables {
        // Extend is going to turn this from a Vec2 to a Vec3
        transform.translation = position.0.extend(0.);
        transform.rotation = facing.0;
    }
}

pub fn enforce_movement_limits(
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

pub fn update_positions(movement_variables: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in movement_variables {
        position.0 += velocity.0;
    }
}
