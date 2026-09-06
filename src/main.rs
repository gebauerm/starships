// Domains: combat + projectiles + shooting systems, movement system, scoring system, ship systems


// TODO: add boost system for movement
// TODO: add floating objects for level building
// TODO: add winnig points
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

mod config;
pub mod player_config;

// Component Movement
#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

// Component Movement
#[derive(Component, Default)]
#[require(Transform)]
struct Facing(Quat);
impl Facing {
    fn to_angle(&self) -> f32 {
        // given in radiants
        let (axis, angle) = self.0.to_axis_angle();
        let angle = angle * axis.z;
        angle
    }
}

// Component Combat
#[derive(Component, Default)]
struct Health(f32);


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


// Component Movement
#[derive(Component, Default)]
#[require(Facing)]
struct Thrust(Vec2);


// Component Movement
#[derive(Component, Default, Debug)]
struct Velocity(Vec2);

impl Velocity {
    // This is used to initialize shots
    fn from_facing(facing: &Facing) -> Self {
        let angle = facing.to_angle();
        let thrust = vec_from_angle(angle) * config::MAX_SHOT_VELOCITY;
        Self(thrust)
    }
}


// Component Projectiles
#[derive(Component, Default)]
#[require(Position, Thrust = Thrust(Vec2::ZERO), Velocity = Velocity(Vec2::ZERO), Collider=Collider(Rectangle::new(config::SHOT_SIZE, config::SHOT_SIZE)), Health = Health(1.))]
struct Shot;


// Component Combat
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    FRONT,
    BACK,
    SIDE
}

// Component Combat
#[derive(Component, Default)]
struct Collider(Rectangle);
impl Collider {
    fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}

// Component Ship
#[derive(Component, Default)]
#[require(Position, Thrust = Thrust(Vec2::ZERO), Velocity = Velocity(Vec2::ZERO), Health = Health(config::SHIP_HEALTH),
Collider = Collider(Rectangle::new(config::SHIP_SIZE-10., config::SHIP_SIZE-10.)), ShootDelayTimer= ShootDelayTimer::default(), BoostDelayTimer = BoostDelayTimer::default(), BoostDurationTimer=BoostDurationTimer::default())]
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
    position: Position,
    facing: Facing,
    health: Health,
    shoot_delay_timer: ShootDelayTimer,
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
            position: Position(position),
            facing: Facing(facing),
            health: Health(config::SHIP_HEALTH),
            shoot_delay_timer: ShootDelayTimer::default(),
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


// Component timers
#[derive(Component)]
struct BoostDelayTimer {
    timer: Timer,
}
impl BoostDelayTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(config::BOOST_DELAY, TimerMode::Once);
        // timer.tick(Duration::from_secs_f32(BOOST_DELAY + 1.));
        timer.finish();
        Self { timer: timer }
    }
}


// Component timers
#[derive(Component)]
struct BoostDurationTimer {
    timer: Timer,
}
impl BoostDurationTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(config::BOOST_DURATION, TimerMode::Once),
        }
    }
}


// Component timers
#[derive(Component)]
struct ShootDelayTimer {
    timer: Timer,
}
impl ShootDelayTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(config::MAX_SHOT_DELAY, TimerMode::Once),
        }
    }
}


// Movement
fn vec_from_angle(angle: f32) -> Vec2 {
    Vec2::from_angle(angle).rotate(Vec2::Y).normalize()
}


// Movement
fn project_positions(mut positionables: Query<(&mut Transform, &Position, &Facing)>) {
    for (mut transform, position, facing) in &mut positionables {
        // Extend is going to turn this from a Vec2 to a Vec3
        transform.translation = position.0.extend(0.);
        transform.rotation = facing.0;
    }
}


// timers
fn tick_timers(
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


// Movement
fn enforce_movement_limits(
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


// Mmovement systems
fn update_positions(movement_variables: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in movement_variables {
        position.0 += velocity.0;
    }
}


// shooting systems
fn spawn_shots(
    event: On<Shoot>,
    mut commands: Commands,
    mut variables: Query<
        (&Position, &Facing, &mut ShootDelayTimer, &PlayerColor),
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
                + vec_from_angle(facing.to_angle())
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

    if defender_health.0 <= 0. {
        Scored { entity: attacker };
    }

    if attacker_health.0 <= 0. {
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


// combat systems
fn collision_with_shot(player: Aabb2d, shot: Aabb2d) -> Option<Collision> {
    if !player.intersects(&shot) {
        return None;
    }
    Some(Collision::FRONT)
}

// combat systems
fn handle_shot_hits(
    mut commands: Commands,
    player_variables: Query<(&Position, &Collider, &mut Health), With<PlayerControls>>,
    shot_variables: Query<(&Position, &Collider, Entity), With<Shot>>,
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


// combat systems
fn clear_dead_stuff(mut commands: Commands, entity_variables: Query<(Entity, &Health)>) {
    for (entity, health) in entity_variables {
        if health.0 <= 0. {
            commands.entity(entity).despawn();
        }
    }
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
                tick_timers.before(handle_player_inputs),
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
