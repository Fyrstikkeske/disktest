use macroquad::prelude::*;



pub async fn texture_manager() -> crate::render::Texturemanager{
    let texturemanager = crate::render::Texturemanager{
        dirt: load_texture("textures/dirt.png").await.unwrap(),
        imposter: load_texture("textures/imposter.png").await.unwrap(),
        stone: load_texture("textures/stone.png").await.unwrap(),
        grass: load_texture("textures/grass.png").await.unwrap(),
    };
    //todo, automate, NEVER HAHAHAHAHA
    texturemanager.stone.set_filter(FilterMode::Nearest);
    texturemanager.dirt.set_filter(FilterMode::Nearest);
    texturemanager.grass.set_filter(FilterMode::Nearest);
    texturemanager.imposter.set_filter(FilterMode::Nearest);

    return texturemanager;
}