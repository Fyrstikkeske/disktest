use macroquad::prelude::*;

pub struct Texturemanager{
	pub dirt: Texture2D,
	pub imposter: Texture2D,
	pub stone: Texture2D,
	pub grass: Texture2D,
    pub hotbar: Texture2D,
    pub pickaxe: Texture2D,
    pub simple_spaceship: Texture2D,
}

pub async fn texture_manager() -> Texturemanager{
    let texturemanager = Texturemanager{
        dirt: load_texture("textures/dirt.png").await.unwrap(),
        imposter: load_texture("textures/imposter.png").await.unwrap(),
        stone: load_texture("textures/stone.png").await.unwrap(),
        grass: load_texture("textures/grass.png").await.unwrap(),
        hotbar: load_texture("textures/hotbar.png").await.unwrap(),
        pickaxe: load_texture("textures/iron_pickaxe.png").await.unwrap(),
        simple_spaceship: load_texture("textures/rocketship.png").await.unwrap(),
    };
    //todo, automate, NEVER HAHAHAHAHA
    texturemanager.stone.set_filter(FilterMode::Nearest);
    texturemanager.dirt.set_filter(FilterMode::Nearest);
    texturemanager.grass.set_filter(FilterMode::Nearest);
    texturemanager.imposter.set_filter(FilterMode::Nearest);
    texturemanager.hotbar.set_filter(FilterMode::Nearest);
    texturemanager.pickaxe.set_filter(FilterMode::Nearest);
    texturemanager.simple_spaceship.set_filter(FilterMode::Nearest);
    return texturemanager;
}