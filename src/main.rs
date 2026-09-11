//! A fast-paced 2-player spaceship battle arcade game built with Bevy.
//!
//! Crate layout:
//! - [`ship`]: the player ship, its controls and physics
//! - [`projectiles`]: shots and the shooting system
//! - [`combat`]: hit points, colliders, collision resolution
//! - [`movement`]: kinematics (`Position`, `Facing`, `Velocity`) and motion systems
//! - [`timers`]: cooldown timers
//! - [`score`]: scoring and match reset
//! - [`config`] / [`player_config`]: tuning constants and player bindings
#![warn(missing_docs)]
use bevy::prelude::*;
use combat::{clear_dead_stuff, handle_shot_hits};
use movement::*;
use ship::{PlayerControls, Ship, handle_player_inputs, spawn_players, update_ship_velocity};
use sprites::load_sprites;

mod combat;
mod config;
mod movement;
pub mod player_config;
mod projectiles;
mod score;
mod ship;
mod sprites;
mod timers;

/// Initializing a 2D camera.
fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}

/// Starts and setups the Application. Spawn Logics an Game Systems, as well as observers are run.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(score::Score {
            attacker: 0,
            defender: 0,
        })
        .add_systems(
            Startup,
            (
                spawn_camera,
                load_sprites.before(spawn_players),
                spawn_players,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                project_positions,
                timers::tick_timers.before(handle_player_inputs),
                handle_player_inputs.before(update_ship_velocity),
                update_ship_velocity.before(project_positions),
                update_positions.after(update_ship_velocity),
                enforce_movement_limits.after(update_positions),
                handle_shot_hits.after(enforce_movement_limits),
                //detect_player_destruction.before(clear_dead_stuff),
                clear_dead_stuff.after(handle_shot_hits),
            ),
        )
        .add_observer(projectiles::spawn_shots)
        .add_observer(score::update_score)
        //.add_observer(reset_game)
        .run();
}
