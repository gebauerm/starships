pub const SHIP_HEALTH: f32 = 100.0;
pub const SHIP_SIZE: f32 = 30.0;
pub const SHIP_THRUST: f32 = 0.2;
pub const MAX_SHIP_VELOCITY: f32 = 4.0;
pub const SHIP_ROTATION_SPEED: f32 = f32::to_radians(3.0);
/// TODO: need to add a boost mechanic
pub const BOOST_THRUST: f32 = 20.0;
pub const SHIP_BREAKS: f32 = SHIP_THRUST * 0.3;
pub const BOOST_DELAY: f32 = 4.0;
pub const BOOST_DURATION: f32 = 1.0;

pub const MAX_SHOT_DELAY: f32 = 0.5;
pub const MAX_SHOT_VELOCITY: f32 = 12.0;
pub const SHOT_SIZE: f32 = 25.0;
pub const BACK_HIT_THRESHOLD: f32 = 0.8;
pub const FRONT_HIT_THRESHOLD: f32 = -0.2;
pub const FRONT_SHOT_DMG: f32 = 10.0;
pub const SIDE_SHOT_DMG: f32 = 20.0;
pub const BACK_SHOT_DMG: f32 = 40.0;
pub const SHOOT_OFFSET: f32 = 10.0;
