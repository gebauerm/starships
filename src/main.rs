// TODO: add boost system for movement
// TODO: add floating objects for level building
// TODO: add winnig points
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

const SHIP_ROTATION_SPEED: f32 = f32::to_radians(3.0);
const SHIP_THRUST: f32 = 0.2;
const SHIP_HEALTH: f32 = 100.;
const SHIP_SIZE: f32 = 30.;
const MAX_SHIP_VELOCITY: f32 = 4.;
const BOOST_THRUST: f32 = 20.;
const SHIP_BREAKS: f32 = SHIP_THRUST * 0.3;
const BOOST_DELAY: f32 = 4.;
const BOOST_DURATION: f32 = 1.;

const MAX_SHOT_DELAY: f32 = 0.5;
const MAX_SHOT_VELOCITY: f32 = 12.;
const SHOT_SIZE: f32 = 25.;
const SHOT_DMG: f32 = 40.;
const SHOOT_OFFSET: f32 = 10.;

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

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

#[derive(Component, Default)]
struct Health(f32);

#[derive(EntityEvent)]
struct Scored {
    entity: Entity,
}

#[derive(Resource)]
struct Score {
    attacker: u8,
    defender: u8,
}

#[derive(Component, Default)]
#[require(Facing)]
struct Thrust(Vec2);

#[derive(Component, Default, Debug)]
struct Velocity(Vec2);

impl Velocity {
    // This is used to initialize shots
    fn from_facing(facing: &Facing) -> Self {
        let angle = facing.to_angle();
        let thrust = vec_from_angle(angle) * MAX_SHOT_VELOCITY;
        Self(thrust)
    }
}

#[derive(Component, Default)]
#[require(Position, Thrust = Thrust(Vec2::ZERO), Velocity = Velocity(Vec2::ZERO), Collider=Collider(Rectangle::new(SHOT_SIZE, SHOT_SIZE)), Health = Health(1.))]
struct Shot;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    FRONT,
    BACK,
    SIDE,
}

#[derive(Component, Default)]
struct Collider(Rectangle);
impl Collider {
    fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}

#[derive(Component, Default)]
#[require(Position, Thrust = Thrust(Vec2::ZERO), Velocity = Velocity(Vec2::ZERO), Health = Health(SHIP_HEALTH),
Collider = Collider(Rectangle::new(SHIP_SIZE-10., SHIP_SIZE-10.)), ShootDelayTimer= ShootDelayTimer::default(), BoostDelayTimer = BoostDelayTimer::default(), BoostDurationTimer=BoostDurationTimer::default())]
struct Ship;

#[derive(Component, Default)]
struct PlayerColor(Color);

#[derive(Component)]
#[require(Ship)]
struct Player {
    acc: KeyCode,
    reverse: KeyCode,
    left: KeyCode,
    right: KeyCode,
    fire: KeyCode,
    boost: KeyCode,
    thrust_input: f32,
    rotation_input: f32,
}
impl Player {
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
}

#[derive(Component)]
#[require(Player= Player::new(
            KeyCode::ArrowUp,
            KeyCode::ArrowDown,
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
            KeyCode::Numpad0,
            KeyCode::NumpadComma,
        ), PlayerColor(Color::srgb(1., 0.5, 0.)))]
struct Attacker;

#[derive(Component)]
#[require(Player = Player::new(
            KeyCode::KeyW,
            KeyCode::KeyS,
            KeyCode::KeyA,
            KeyCode::KeyD,
            KeyCode::Space,
            KeyCode::ShiftLeft,
        ), PlayerColor(Color::srgb(0.1, 0.7, 1.)))]
struct Defender;

#[derive(EntityEvent)]
struct Shoot {
    #[event_target]
    shooter: Entity,
}

#[derive(Component)]
struct BoostDelayTimer {
    timer: Timer,
}
impl BoostDelayTimer {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(BOOST_DELAY, TimerMode::Once);
        // timer.tick(Duration::from_secs_f32(BOOST_DELAY + 1.));
        timer.finish();
        Self { timer: timer }
    }
}

#[derive(Component)]
struct BoostDurationTimer {
    timer: Timer,
}
impl BoostDurationTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(BOOST_DURATION, TimerMode::Once),
        }
    }
}

#[derive(Component)]
struct ShootDelayTimer {
    timer: Timer,
}
impl ShootDelayTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(MAX_SHOT_DELAY, TimerMode::Once),
        }
    }
}

fn vec_from_angle(angle: f32) -> Vec2 {
    Vec2::from_angle(angle).rotate(Vec2::Y).normalize()
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position, &Facing)>) {
    for (mut transform, position, facing) in &mut positionables {
        // Extend is going to turn this from a Vec2 to a Vec3
        transform.translation = position.0.extend(0.);
        transform.rotation = facing.0;
    }
}

fn tick_timers(
    timers: Query<&mut ShootDelayTimer, With<Player>>,
    boost_duration: Query<&mut BoostDurationTimer, With<Player>>,
    boost_delay: Query<&mut BoostDelayTimer, With<Player>>,
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

fn spawn_players(mut commands: Commands, window: Single<&Window>, asset_server: Res<AssetServer>) {
    let half_window_size = window.resolution.size() / 2.;
    let padding = 20.;

    let ship_img = asset_server.load("player.png");
    let mut sprite = Sprite::from_image(ship_img.clone());
    sprite.custom_size = Some(Vec2::new(SHIP_SIZE, SHIP_SIZE));

    let attacker_pos = Vec2::new(half_window_size.x - padding, 0.);
    let defender_pos = Vec2::new(-half_window_size.x + padding, 0.);
    let attacker_facing =
        Quat::from_rotation_z((defender_pos - attacker_pos).to_angle() - FRAC_PI_2);

    // TODO: color setting needs refactoring (keep the coloring of the shots in mind)
    let attacker_color = PlayerColor(Color::srgb(1., 0.5, 0.));
    let defender_color = PlayerColor(Color::srgb(0., 0.5, 1.));
    let mut attacker_sprite = sprite.clone();
    attacker_sprite.color = attacker_color.0;
    let mut defender_sprite = sprite;
    defender_sprite.color = defender_color.0;

    commands.spawn((
        Attacker,
        Ship,
        attacker_sprite,
        Position(attacker_pos),
        Facing(attacker_facing.clone()),
        Health(SHIP_HEALTH),
    ));

    commands.spawn((
        Defender,
        Ship,
        defender_sprite,
        Position(defender_pos),
        Facing(attacker_facing.inverse()),
        Health(SHIP_HEALTH),
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 0.)));
}

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

fn handle_player_inputs(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    players: Query<(&mut Player, Entity), With<Ship>>,
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

fn update_ship_velocity(
    variables: Query<(&mut Velocity, &mut Thrust, &mut Facing, &Player), With<Ship>>,
) {
    for (mut velocity, mut thrust, mut facing, player) in variables {
        let (axis, angle) = facing.0.to_axis_angle();
        let mut angle = angle * axis.z;
        angle += SHIP_ROTATION_SPEED * player.rotation_input;
        facing.0 = Quat::from_rotation_z(angle);

        thrust.0 = Vec2::from_angle(angle).rotate(Vec2::Y) * SHIP_THRUST * player.thrust_input;
        if player.thrust_input < 0. {
            thrust.0 = velocity.0.normalize() * SHIP_BREAKS * player.thrust_input;
        }
        velocity.0 += thrust.0;
        if velocity.0.length_squared() > MAX_SHIP_VELOCITY.powi(2) {
            velocity.0 = velocity.0.normalize() * MAX_SHIP_VELOCITY;
        }
    }
}

fn update_positions(movement_variables: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in movement_variables {
        position.0 += velocity.0;
    }
}

fn spawn_shots(
    event: On<Shoot>,
    mut commands: Commands,
    mut variables: Query<(&Position, &Facing, &mut ShootDelayTimer, &PlayerColor), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let attacker_img = asset_server.load("shot.png");
    let mut sprite = Sprite::from_image(attacker_img);
    sprite.custom_size = Some(Vec2::new(SHOT_SIZE, SHOT_SIZE));

    if let Ok((position, facing, mut config, player_color)) = variables.get_mut(event.shooter) {
        if config.timer.is_finished() {
            sprite.color = player_color.0;
            let position =
                position.0 + vec_from_angle(facing.to_angle()) * (SHIP_SIZE / 2. + SHOOT_OFFSET);
            commands.spawn((
                Shot,
                Transform::from_translation(position.extend(0.)),
                Position(position),
                Velocity::from_facing(facing),
                sprite,
            ));
            config.timer.reset();
        }
    }
}

fn detect_player_destruction(
    attacker: Single<(Entity, &Health), With<Attacker>>,
    defender: Single<(Entity, &Health), With<Defender>>,
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

fn update_score(
    event: On<Scored>,
    mut score: ResMut<Score>,
    attacker: Query<Entity, With<Attacker>>,
    defender: Query<Entity, With<Defender>>,
) {
    if attacker.get(event.entity).is_ok() {
        score.attacker += 1;
    }
    if defender.get(event.entity).is_ok() {
        score.defender += 1;
    }
}

fn reset_game(
    _event: On<Scored>,
    window: Single<&Window>,
    attacker_variables: Single<(&mut Facing, &mut Position, &mut Health), With<Attacker>>,
    defender_variables: Single<(&mut Facing, &mut Position, &mut Health), With<Defender>>,
) {
    // TODO: this is copied code. This needs to be abstracted in a propper way.
    let half_window_size = window.resolution.size() / 2.;
    let padding = 20.;

    let attacker_pos = Vec2::new(half_window_size.x - padding, 0.);
    let defender_pos = Vec2::new(-half_window_size.x + padding, 0.);
    let attacker_facing =
        Quat::from_rotation_z((defender_pos - attacker_pos).to_angle() - FRAC_PI_2);
    let defender_facing = attacker_facing.inverse();

    let (attacker_facing, attacker_position, attacker_health) = attacker_variables.into_inner();
    let (defender_facing, defender_position, defender_health) = defender_variables.into_inner();

    // TODO: please make the assignments more efficient.
}

fn collision_with_shot(player: Aabb2d, shot: Aabb2d) -> Option<Collision> {
    if !player.intersects(&shot) {
        return None;
    }
    Some(Collision::FRONT)
}

fn handle_shot_hits(
    mut commands: Commands,
    player_variables: Query<(&Position, &Collider, &mut Health), With<Player>>,
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
                        health.0 -= SHOT_DMG;
                    }
                    Collision::BACK => {
                        health.0 -= SHOT_DMG;
                    }
                    Collision::SIDE => {
                        health.0 -= SHOT_DMG;
                    }
                }
                commands.entity(shot).despawn();
            }
        }
    }
}

fn clear_dead_stuff(
    mut commands: Commands,
    entity_variables: Query<(Entity, &Health)>,
) {
    for (entity, health) in entity_variables {
        if health.0 <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score {
            attacker: 0,
            defender: 0,
        })
        .add_systems(Startup, (spawn_camera, spawn_players))
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
