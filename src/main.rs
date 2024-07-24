use macroquad::prelude::*;
use num_complex::Complex;

mod render;
mod collision;
mod chunk;


const ARRX:usize = 100000;
const ARRY:usize = 3;


#[macroquad::main("Torus")]
async fn main() {
	let mut hyperboria: chunk::World = chunk::World{x_size: ARRX, y_size: ARRY, blocks: &mut [chunk::BlockType::Stone; ARRX*ARRY]};
	let mut player = collision::DynRect{rect:Rect{x:1.0, y: 100.0, w: 1.0, h:1.0}, velocity: Vec2::ZERO};

	let mut world_offset_rotation:f32 = 0.0;
	let mut world_offset_height:f32 = 6.0;
	let world_offset_global_x:f32 = 960.0;
	let mut world_offset_global_y:f32 = 540.0;

    println!("Hello, universe!");
    

	let moon:chunk::World = chunk::World{x_size: 25, y_size: 25, blocks: &mut [chunk::BlockType::Stone; 25*25]};

	for i in 0..hyperboria.x_size{
		hyperboria.blocks[(hyperboria.x_size * hyperboria.y_size)-(i+1)] = chunk::BlockType::Grass;
	}
	hyperboria.blocks[9899] = chunk::BlockType::Air;
	hyperboria.blocks[9999] = chunk::BlockType::Air;
	hyperboria.blocks[9998] = chunk::BlockType::Air;

	let texturemanager = render::Texturemanager{
		dirt: load_texture("textures/dirt.png").await.unwrap(),
		imposter: load_texture("textures/imposter.png").await.unwrap(),
		stone: load_texture("textures/stone.png").await.unwrap(),
		grass: load_texture("textures/grass.png").await.unwrap(),
	};

	texturemanager.stone.set_filter(FilterMode::Nearest);
	texturemanager.dirt.set_filter(FilterMode::Nearest);
	texturemanager.grass.set_filter(FilterMode::Nearest);


    loop{
    	clear_background(BLACK);
		let delta = get_frame_time();


		if is_key_down(KeyCode::Right) {
            world_offset_rotation -= std::f32::consts::TAU/hyperboria.x_size as f32;
        }

		if is_key_down(KeyCode::Left) {
            world_offset_rotation += std::f32::consts::TAU/hyperboria.x_size as f32;
        }

		if is_key_down(KeyCode::Down) {
            world_offset_height += 100.*(std::f32::consts::TAU/hyperboria.x_size as f32);
        }

		if is_key_down(KeyCode::Up) {
            world_offset_height -= 100.*(std::f32::consts::TAU/hyperboria.x_size as f32);
        }

		if is_key_down(KeyCode::A) {
            player.velocity.x -= 40.0 * delta;
        }

		if is_key_down(KeyCode::D) {
            player.velocity.x += 40.0 * delta;
        }

		if is_key_down(KeyCode::W) {
			world_offset_global_y -= 1000.;
        }

		if is_key_down(KeyCode::S) {
			world_offset_global_y += 1000.;
        }

		if is_key_pressed(KeyCode::Space) {
			player.velocity.y += 10.00;
        }

		
		collision::dynamic_rectangle_vs_world(&delta, &mut player, &mut hyperboria);

		let normalisedplayerx = (player.rect.x *2.0 /hyperboria.x_size as f32 -1.0) * std::f32::consts::PI;
		let normalisedplayery = (player.rect.y - hyperboria.y_size as f32) *((std::f32::consts::PI*2.)/hyperboria.x_size as f32);

		let mut playercomplex = Complex{re:normalisedplayery + world_offset_height, im:normalisedplayerx + world_offset_rotation};
		playercomplex = Complex::exp(playercomplex);
		let player_node_x = playercomplex.re;
		let player_node_y = playercomplex.im;
		let player_size = f32::sqrt(f32::powf(playercomplex.re,2.)+f32::powf(playercomplex.im,2.)) *((std::f32::consts::PI*2.)/ARRX as f32);


		render::render_world(&hyperboria, &texturemanager, world_offset_height, world_offset_rotation, world_offset_global_x, world_offset_global_y);

		render::render_world(&moon, &texturemanager, 4.0, 0.0, 100.0, 200.0);

		player.rect.x = player.rect.x + (player.velocity.x * delta);
    	player.rect.y = player.rect.y + (player.velocity.y * delta);

		player.velocity.x = player.velocity.x * 0.96;
		player.velocity.y -= 9.81* delta;

		if player.velocity.x.abs() < 4.{player.velocity.x = player.velocity.x * 0.89;};

		draw_texture_ex(
				&texturemanager.imposter,
				player_node_x - player_size/2. + world_offset_global_x,
				player_node_y - player_size/2. + world_offset_global_y,
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(player_size,player_size*player.rect.h)),
					rotation: player_node_y.atan2(player_node_x)+std::f32::consts::PI/2.,
					..Default::default()
				}
			);
    	next_frame().await
    }
}
