use macroquad::{
	color::WHITE,
	math::{vec2, Vec2},
	texture::{draw_texture_ex, 
		DrawTextureParams,
		Texture2D,
		}
	};



use num_complex::Complex;

use crate::chunk::{BlockType, World};


pub struct Texturemanager{
	pub dirt: Texture2D,
	pub imposter: Texture2D,
	pub stone: Texture2D,
	pub grass: Texture2D,
}



pub fn render_world
(
	world : &World, 
	texturemanager: &Texturemanager,
	world_offset_height: f32, 
	world_offset_rotation: f32,
	world_offset_global_x: f32,
	world_offset_global_y: f32,
){	

	for i in 0..(world.x_size*world.y_size){
		let x = i%world.x_size;
		let y = i/world.x_size;


			//should be obvius
			match world.blocks[i] {
				BlockType::Air => {continue}
				BlockType::Marvin => {continue}
				_ => {}
			}

			if world.blocks[i] == BlockType::Air{
				continue;
			}

			let normalised_block_position = Vec2{
				x:(x as f32 *2.0 /world.x_size as f32 -1.0) * std::f32::consts::PI,
				y: ((y) as f32 - world.y_size as f32) *((std::f32::consts::PI*2.)/world.x_size as f32)
			};



			let pre_complex_block_position = Complex{re:normalised_block_position.y + world_offset_height, im:normalised_block_position.x + world_offset_rotation};
			let complex_block_position = Complex::exp(pre_complex_block_position);
			let block_x = complex_block_position.re;
			let block_y = complex_block_position.im;


			let size = f32::sqrt(f32::powf(block_x,2.)+f32::powf(block_y,2.)) *((std::f32::consts::PI*2.)/world.x_size as f32);

			let texture_to_use:&Texture2D;
			let rotation = block_y.atan2(block_x) +std::f32::consts::PI/2.;


			match world.blocks[i] {
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