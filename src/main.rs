use chunk::Planet;
use macroquad::prelude::*;
use num_complex::Complex;

mod render;
mod collision;
mod chunk;



#[macroquad::main("Torus")]
async fn main() {
	println!("Hello, universe!");

	let terra = Planet{
		name: "Terra",
		space_position: &mut{Vec2{x: 0.0, y: 0.0}},
		size: IVec2 { x: 100, y: 50 },
		rotation: 0.0,
	};

	let mut player:collision::MovableEntity = collision::MovableEntity{
		dynrect: collision::DynRect{rect:Rect{x:1.0, y: 100.0, w: 1.0, h:1.0}, velocity: Vec2::ZERO},
		planet: Some(&terra),
	};

    

	let texturemanager = render::Texturemanager{
		dirt: load_texture("textures/dirt.png").await.unwrap(),
		imposter: load_texture("textures/imposter.png").await.unwrap(),
		stone: load_texture("textures/stone.png").await.unwrap(),
		grass: load_texture("textures/grass.png").await.unwrap(),
	};
	//todo, automate
	texturemanager.stone.set_filter(FilterMode::Nearest);
	texturemanager.dirt.set_filter(FilterMode::Nearest);
	texturemanager.grass.set_filter(FilterMode::Nearest);
	texturemanager.imposter.set_filter(FilterMode::Nearest);


    loop{
    	clear_background(BLACK);
		let delta = get_frame_time();

		//add in camera, use terraria test as base since its just that magically good



		// fix this in chunk refactor
		collision::dynamic_rectangle_vs_world(&delta, &mut player.dynrect, &mut player.planet);



		// This should be put in its own function, could alse be used as a base for render_world()
		let normalisedplayerx = (player.rect.x *2.0 /hyperboria.x_size as f32 -1.0) * std::f32::consts::PI;
		let normalisedplayery = (player.rect.y - hyperboria.y_size as f32) *((std::f32::consts::PI*2.)/hyperboria.x_size as f32);
		let mut playercomplex = Complex{re:normalisedplayery + world_offset_height, im:normalisedplayerx + world_offset_rotation};
		playercomplex = Complex::exp(playercomplex);
		let player_node_x = playercomplex.re;
		let player_node_y = playercomplex.im;
		let player_size = f32::sqrt(f32::powf(playercomplex.re,2.)+f32::powf(playercomplex.im,2.)) *((std::f32::consts::PI*2.)/ARRX as f32);



		//make it so that i only render the world the player is on, The situation in where he can see 2 planets at the same time should never happen
		//something like this render_world(player.planet)
		render::render_world(&hyperboria, &texturemanager, world_offset_height, world_offset_rotation, world_offset_global_x, world_offset_global_y);
		render::render_world(&moon, &texturemanager, 4.0, 0.0, 100.0, 200.0);

		//movement should be its own function yet again.
		player.rect.x = player.rect.x + (player.velocity.x * delta);
    	player.rect.y = player.rect.y + (player.velocity.y * delta);
		player.velocity.x = player.velocity.x * 0.96;
		player.velocity.y -= 9.81* delta;
		if player.velocity.x.abs() < 4.{player.velocity.x = player.velocity.x * 0.89;};


		//Def its own function, should be adapted for any entity
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






/*	TODO: Put this in its own function, and separate. this is way to big to have in the main function

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
 */