// Domains: combat + projectiles + shooting systems, movement system, scoring system, ship systems


// TODO: add boost system for movement
// TODO: add floating objects for level building
// TODO: add winnig points
use bevy::prelude::*;
use movement::*;
use combat::{Health, clear_dead_stuff, handle_shot_hits};
use ship::{PlayerControls, Ship, Shipsprite, load_sprites, handle_player_inputs, spawn_players, update_ship_velocity};


mod config;
mod timers;
mod movement;
mod combat;
mod projectiles;
mod ship;
pub mod player_config;


// Event Scoring
#[derive(EntityEvent)]
struct Scored {
    entity: Entity,
}


// Reesource Scoring
#[derive(Resource)]
struct Score {
    attacker: u8,
    defender: u8,
}


// App Setup
fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}


// scoring systems
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


// scoring system
fn update_score(
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


// scoring system
fn reset_game(
    _event: On<Scored>,
    window: Single<&Window>,
    commands: Commands,
    ship_sprite: Res<Shipsprite>,
) {
    ship::spawn_players(commands, window, ship_sprite);
}



// App Setup
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score {
            attacker: 0,
            defender: 0,
        })
        .add_systems(Startup, (spawn_camera, load_sprites.before(spawn_players), spawn_players))
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
        .add_observer(update_score)
        //.add_observer(reset_game)
        .run();
}
