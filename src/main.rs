// Domains: combat + projectiles + shooting systems, movement system, scoring system, ship systems


// TODO: add boost system for movement
// TODO: add floating objects for level building
// TODO: add winnig points
use bevy::prelude::*;
use movement::*;
use combat::{Health, clear_dead_stuff, handle_shot_hits};


mod config;
mod timers;
mod movement;
mod combat;
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




// Component Projectiles
#[derive(Component, Default)]
#[require(Position, movement::Thrust = movement::Thrust(Vec2::ZERO), movement::Velocity = movement::Velocity(Vec2::ZERO), combat::Collider=combat::Collider(Rectangle::new(config::SHOT_SIZE, config::SHOT_SIZE)), combat::Health = combat::Health(1.))]
struct Shot;

// Component Ship
#[derive(Component, Default)]
#[require(movement::Position, movement::Thrust = movement::Thrust(Vec2::ZERO), movement::Velocity = movement::Velocity(Vec2::ZERO), combat::Health = combat::Health(config::SHIP_HEALTH),
combat::Collider = combat::Collider(Rectangle::new(config::SHIP_SIZE-10., config::SHIP_SIZE-10.)), timers::ShootDelayTimer= timers::ShootDelayTimer::default(), timers::BoostDelayTimer = timers::BoostDelayTimer::default(), timers::BoostDurationTimer=timers::BoostDurationTimer::default())]
struct Ship;


// Component ship look
#[derive(Component, Default)]
struct PlayerColor(Color);


// Ressource ship look
#[derive(Resource)]
struct Shipsprite(Sprite);


// App Setup
fn load_ship_sprite(asset_server: &AssetServer) -> Shipsprite {
    let ship_img = asset_server.load("player.png");
    let mut sprite = Sprite::from_image(ship_img.clone());
    sprite.custom_size = Some(Vec2::new(config::SHIP_SIZE, config::SHIP_SIZE));
    Shipsprite(sprite)
}

// App Setup
fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(load_ship_sprite(&asset_server));
}


// Ship Input
#[derive(Component)]
#[require(Ship)]
struct PlayerControls {
    acc: KeyCode,
    reverse: KeyCode,
    left: KeyCode,
    right: KeyCode,
    fire: KeyCode,
    boost: KeyCode,
    thrust_input: f32,
    rotation_input: f32,
}
impl PlayerControls {
    fn new(
        acc: KeyCode,
        reverse: KeyCode,
        left: KeyCode,
        right: KeyCode,
        fire: KeyCode,
        boost: KeyCode,
    ) -> Self {
        Self {
            acc,
            reverse,
            left,
            right,
            fire,
            boost,
            thrust_input: 0.,
            rotation_input: 0.,
        }
    }

    fn from_config(player_config: &player_config::PlayerConfig) -> Self {
        Self {
            acc: player_config.acc,
            reverse: player_config.reverse,
            left: player_config.left,
            right: player_config.right,
            fire: player_config.fire,
            boost: player_config.boost,
            thrust_input: 0.,
            rotation_input: 0.,
        }
    }
}
impl Default for PlayerControls {
    fn default() -> Self {
        Self {
            acc: KeyCode::KeyW,
            reverse: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            fire: KeyCode::Space,
            boost: KeyCode::ShiftLeft,
            thrust_input: 0.,
            rotation_input: 0.,
        }
    }
}


// Ship spawning
#[derive(Bundle)]
struct PlayerBundle {
    // TODO: move this into player_config, together with controls. Players should be spawne directly from player_config.
    controls: PlayerControls,
    sprite: Sprite,
    position: movement::Position,
    facing: movement::Facing,
    health: Health,
    shoot_delay_timer: timers::ShootDelayTimer,
    ship: Ship,
    color: PlayerColor,
}

impl PlayerBundle {
    fn new(
        player_config: &player_config::PlayerConfig,
        sprite: Sprite,
        window: &Single<&Window>,
    ) -> Self {
        let (position, facing) = player_config.starting_positions(window);
        let sprite = player_config.color_sprites(sprite);
        let controls = PlayerControls::from_config(&player_config);
        Self {
            controls: controls,
            sprite: sprite,
            position: movement::Position(position),
            facing: movement::Facing(facing),
            health: Health(config::SHIP_HEALTH),
            shoot_delay_timer: timers::ShootDelayTimer::default(),
            ship: Ship,
            color: PlayerColor(player_config.color)
        }
    }
}


// Event Shooting
#[derive(EntityEvent)]
struct Shoot {
    #[event_target]
    shooter: Entity,
}







// ship spawning
fn spawn_player<C: Component>(
    player_config: &player_config::PlayerConfig,
    window: &Single<&Window>,
    sprite: Sprite,
    role_marker: C,
) -> (PlayerBundle, impl Component) {
    let player_bundle = PlayerBundle::new(player_config, sprite, window);

    (player_bundle, role_marker)
}


// ship spawning
fn spawn_players(mut commands: Commands, window: Single<&Window>, ship_sprite: Res<Shipsprite>) {
    for player_config in player_config::PLAYER_CONFIGS.iter() {
        match player_config.role {
            player_config::PlayerRole::Attacker => {
                let (player_bundle, role) = spawn_player(
                    player_config,
                    &window,
                    ship_sprite.0.clone(),
                    player_config::Attacker,
                );
                commands.spawn((player_bundle, role));
            }
            player_config::PlayerRole::Defender => {
                let (player_bundle, role) = spawn_player(
                    player_config,
                    &window,
                    ship_sprite.0.clone(),
                    player_config::Defender,
                );
                commands.spawn((player_bundle, role));
            }
        }
    }
}


// App Setup
fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}



// ship systems
fn handle_player_inputs(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    players: Query<(&mut PlayerControls, Entity), With<Ship>>,
) {
    for (mut player, entity) in players {
        if keyboard_input.pressed(player.acc) {
            player.thrust_input = 1.;
        } else if keyboard_input.pressed(player.reverse) {
            player.thrust_input = -1.;
        } else if keyboard_input.just_pressed(player.boost) {
            // here implement boost
        }

        if keyboard_input.pressed(player.left) {
            player.rotation_input = 1.;
        } else if keyboard_input.pressed(player.right) {
            player.rotation_input = -1.;
        } else {
            player.rotation_input = 0.;
        }

        if keyboard_input.pressed(player.fire) {
            commands.trigger(Shoot { shooter: entity });
        }
    }
}


// ship systems
fn update_ship_velocity(
    variables: Query<(&mut Velocity, &mut Thrust, &mut Facing, &PlayerControls), With<Ship>>,
) {
    for (mut velocity, mut thrust, mut facing, player) in variables {
        let (axis, angle) = facing.0.to_axis_angle();
        let mut angle = angle * axis.z;
        angle += config::SHIP_ROTATION_SPEED * player.rotation_input;
        facing.0 = Quat::from_rotation_z(angle);

        thrust.0 =
            Vec2::from_angle(angle).rotate(Vec2::Y) * config::SHIP_THRUST * player.thrust_input;
        if player.thrust_input < 0. {
            thrust.0 = velocity.0.normalize() * config::SHIP_BREAKS * player.thrust_input;
        }
        velocity.0 += thrust.0;
        if velocity.0.length_squared() > config::MAX_SHIP_VELOCITY.powi(2) {
            velocity.0 = velocity.0.normalize() * config::MAX_SHIP_VELOCITY;
        }
    }
}


// shooting systems
fn spawn_shots(
    event: On<Shoot>,
    mut commands: Commands,
    mut variables: Query<
        (&movement::Position, &movement::Facing, &mut timers::ShootDelayTimer, &PlayerColor),
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
                + movement::direction_from_angle(facing.to_angle())
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


// scoring systems
fn detect_player_destruction(
    attacker: Single<(Entity, &Health), With<player_config::Attacker>>,
    defender: Single<(Entity, &Health), With<player_config::Defender>>,
) {
    let (attacker, attacker_health) = attacker.into_inner();
    let (defender, defender_health) = defender.into_inner();

    if defender_health.is_dead() {
        Scored { entity: attacker };
    }

    if attacker_health.is_dead() {
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
    mut commands: Commands,
    ship_sprite: Res<Shipsprite>,
) {
    spawn_players(commands, window, ship_sprite);
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
        .add_observer(spawn_shots)
        .add_observer(update_score)
        //.add_observer(reset_game)
        .run();
}
