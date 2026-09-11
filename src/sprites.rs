use bevy::prelude::*;
use crate::config;



#[derive(Resource)]
pub struct ShipSprite(pub Sprite);

#[derive(Resource)]
pub struct ShotSprite(pub Sprite);


fn load_ship_sprite(asset_server: &AssetServer) -> ShipSprite {
    let ship_img = asset_server.load("player.png");
    let mut sprite = Sprite::from_image(ship_img.clone());
    sprite.custom_size = Some(Vec2::new(config::SHIP_SIZE, config::SHIP_SIZE));
    ShipSprite(sprite)
}

pub fn load_shot_sprite(asset_server: &AssetServer) -> ShotSprite{
    let attacker_img = asset_server.load("shot.png");
    let mut sprite = Sprite::from_image(attacker_img);
    sprite.custom_size = Some(Vec2::new(config::SHOT_SIZE, config::SHOT_SIZE));
    ShotSprite(sprite)
}

pub fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(load_ship_sprite(&asset_server));
    commands.insert_resource(load_shot_sprite(&asset_server));
}
