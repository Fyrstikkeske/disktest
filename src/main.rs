use macroquad::prelude::*;
use num_complex::Complex;


#[derive(Clone, Copy, PartialEq)]
enum BlockType {
	Air,
	Stone,
	Dirt,
	Grass,
	Marvin,
}

struct World<'a>{
	x_size: usize,
	y_size: usize,
	blocks: &'a mut [BlockType],
}

struct Texturemanager{
	dirt: Texture2D,
	imposter: Texture2D,
	stone: Texture2D,
	grass: Texture2D,
}

const ARRX:usize = 50;
const ARRY:usize = 50;


#[macroquad::main("Torus")]
async fn main() {
	let mut playerx = 0.0;
	let mut playery = 0.05;
	let mut world_offset_rotation:f32 = 0.0;
	let mut world_offset_height:f32 = 6.0;
	let mut world_offset_global_x:f32 = 960.0;
	let mut world_offset_global_y:f32 = 540.0;

    println!("Hello, world!");

    let hyperboria:World = World{x_size: ARRX, y_size: ARRY, blocks: &mut [BlockType::Air; ARRX*ARRY]};

	let moon:World = World{x_size: 25, y_size: 25, blocks: &mut [BlockType::Stone; 25*25]};

	for i in 0..hyperboria.x_size{
		hyperboria.blocks[(hyperboria.x_size * hyperboria.y_size)-(i+1)] = BlockType::Grass;
	}
	
	let texturemanager = Texturemanager{
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

		if is_key_down(KeyCode::Right) {
            world_offset_rotation -= 0.01;
        }

		if is_key_down(KeyCode::Left) {
            world_offset_rotation += 0.01;
        }

		if is_key_down(KeyCode::Down) {
            world_offset_height += 0.01;
        }

		if is_key_down(KeyCode::Up) {
            world_offset_height -= 0.01;
        }

		if is_key_down(KeyCode::A) {
            playerx -= 0.1;
        }

		if is_key_down(KeyCode::D) {
            playerx += 0.1;
        }

		if is_key_down(KeyCode::W) {
			world_offset_global_y -= 100.;
        }

		if is_key_down(KeyCode::S) {
			world_offset_global_y += 100.;
        }

		if is_key_down(KeyCode::K) {
			playery -= 0.01;
        }

		if is_key_down(KeyCode::L) {
			playery += 0.01;
        }


		let mut playercomplex = Complex{re:playery + world_offset_height, im:playerx + world_offset_rotation};
		playercomplex = Complex::exp(playercomplex);
		let player_node_x = playercomplex.re;
		let player_node_y = playercomplex.im;
		let player_size = f32::sqrt(f32::powf(playercomplex.re,2.)+f32::powf(playercomplex.im,2.)) *((std::f32::consts::PI*2.)/ARRX as f32);


		render_world(&hyperboria, &texturemanager, world_offset_height, world_offset_rotation, world_offset_global_x, world_offset_global_y);

		render_world(&moon, &texturemanager, 4.0, 0.0, 100.0, 200.0);

    	
		draw_texture_ex(
				&texturemanager.imposter,
				player_node_x - player_size/2. + world_offset_global_x,
				player_node_y - player_size/2. + world_offset_global_y,
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(player_size,player_size)),
					rotation: player_node_y.atan2(player_node_x)+std::f32::consts::PI/2.,
					..Default::default()
				}
			);
    	next_frame().await
    }
}


fn render_world
(
	world : &World, 
	texturemanager: &Texturemanager,
	world_offset_height: f32, 
	world_offset_rotation: f32,
	world_offset_global_x: f32,
	world_offset_global_y: f32,
){	

	for x in 0..world.x_size{
		for y in 0..world.y_size{
			

			//should be obvius
			match world.blocks[(y*world.y_size)+x] {
				BlockType::Air => {continue}
				BlockType::Marvin => {continue}
				_ => {}
			}

			if world.blocks[(y*world.y_size)+x] == BlockType::Air{
				continue;
			}

			let normalised_block_position = Vec2{
				x:(x as f32 *2.0 /world.x_size as f32 -1.0) * std::f32::consts::PI,
				y: ((y+1) as f32 - world.y_size as f32) *((std::f32::consts::PI*2.)/world.x_size as f32)
			};



			let pre_complex_block_position = Complex{re:normalised_block_position.y + world_offset_height, im:normalised_block_position.x + world_offset_rotation};


			let complex_block_position = Complex::exp(pre_complex_block_position);
			let block_x = complex_block_position.re;
			let block_y = complex_block_position.im;
			let size = f32::sqrt(f32::powf(block_x,2.)+f32::powf(block_y,2.)) *((std::f32::consts::PI*2.)/world.x_size as f32);

			let texture_to_use:&Texture2D;
			let rotation = block_y.atan2(block_x) +std::f32::consts::PI/2.;


			match world.blocks[(y*world.y_size)+x] {
				BlockType::Dirt => {texture_to_use = &texturemanager.dirt}
				BlockType::Grass => {texture_to_use = &texturemanager.grass}
				BlockType::Stone => {texture_to_use = &texturemanager.stone}
				_ => {continue;}
			}

			draw_texture_ex(
				texture_to_use,
				block_x - size/2. + world_offset_global_x,
				block_y - size/2. + world_offset_global_y,
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(size,size)),
					rotation: rotation,
					..Default::default()
				},
			);	

		}
	}
}

