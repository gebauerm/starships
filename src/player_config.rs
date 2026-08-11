use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub const PLAYER_CONFIGS: [PlayerConfig; 2] = [
    PlayerConfig::new(
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::KeyA,
        KeyCode::KeyD,
        KeyCode::Space,
        KeyCode::ShiftLeft,
        Color::srgb(1., 0.5, 0.),
        PlayerRole::Attacker,
    ),
    PlayerConfig::new(
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
        KeyCode::Numpad0,
        KeyCode::NumpadComma,
        Color::srgb(0.1, 0.5, 1.),
        PlayerRole::Defender,
    ),
];

#[derive(Component)]
pub struct Attacker;

#[derive(Component)]
pub struct Defender;

pub enum PlayerRole {
    Attacker,
    Defender,
}

pub struct PlayerConfig {
    pub acc: KeyCode,
    pub reverse: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub fire: KeyCode,
    pub boost: KeyCode,
    pub color: Color,
    pub role: PlayerRole,
}

impl PlayerConfig {
    const fn new(
        acc: KeyCode,
        reverse: KeyCode,
        left: KeyCode,
        right: KeyCode,
        fire: KeyCode,
        boost: KeyCode,
        color: Color,
        role: PlayerRole,
    ) -> Self {
        Self {
            acc,
            reverse,
            left,
            right,
            fire,
            boost,
            color,
            role,
        }
    }

    pub fn starting_positions(&self, window: &Single<&Window>) -> (Vec2, Quat) {
        let half_window_size = window.resolution.size() / 2.;
        let padding = 20.;

        let (mut pos, mut facing) = (Vec2::ZERO, Quat::default());

        match self.role {
            PlayerRole::Attacker => {
                pos = Vec2::new(half_window_size.x - padding, 0.);
                facing = Quat::from_rotation_z(FRAC_PI_2);
            }
            PlayerRole::Defender => {
                pos = Vec2::new(-half_window_size.x + padding, 0.);
                facing = Quat::from_rotation_z(-FRAC_PI_2);
            }
        }
        (pos, facing)
    }

    pub fn color_sprites(&self, mut sprite: Sprite) -> Sprite {
        sprite.color = self.color;
        sprite
    }
}
