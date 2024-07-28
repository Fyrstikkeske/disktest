use std::collections::HashMap;

use macroquad::{
	color::{Color, WHITE}, math::{vec2, IVec2, Vec2}, shapes::draw_rectangle, texture::{draw_texture_ex, 
		DrawTextureParams,
		Texture2D,
		}
	};



use num_complex::Complex;

use crate::chunk::{BlockType, Planet};


pub struct Texturemanager{
	pub dirt: Texture2D,
	pub imposter: Texture2D,
	pub stone: Texture2D,
	pub grass: Texture2D,
}



//fix
pub fn render_planet_chunks(
	planet : &Planet, 
	point : &Vec2,
	chunks_in_view: &HashMap<IVec2, [BlockType; 1024]>,
	texturemanager: &Texturemanager,
){	

	for chunkinfo in chunks_in_view{
		
        for index in 0..1024{
            let blockcolor:Color = match chunkinfo.1[index] {
                BlockType::Air => Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0},
                BlockType::Marvin => Color { r: 0.5, g: 0.4, b: 0.0, a: 1.0},
                BlockType::Dirt => Color { r: 0.5, g: 0.5, b: 0.1, a: 1.0},
                BlockType::Stone => Color { r: 0.4, g: 0.4, b: 0.45, a: 1.0},
                BlockType::Grass => Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0},
            };

            let chunk_x:i32 = index as i32%32;
            let chunk_y:i32 = index as i32/32;
            
			let x = (chunkinfo.0.x*32) as f32 + chunk_x as f32;
			let y = (chunkinfo.0.y*32) as f32 + chunk_y as f32;

			//now comes the funny part(turning it into a disk)
			
			let normalised_block_position = Vec2{
				x: (x as f32 *2.0 /(planet.size.x as f32*32.0) -1.0) * std::f32::consts::PI,
				y: (y as f32 - (planet.size.y as f32*32.0)) *((std::f32::consts::TAU) / (planet.size.x as f32*32.0) as f32)
			};


			//my brain hurts
			let pre_complex_block_position = Complex{re:normalised_block_position.y + 10.0, im:normalised_block_position.x + *planet.rotation.borrow()};
			let complex_block_position = Complex::exp(pre_complex_block_position);
			let transformed_x = complex_block_position.re;
			let transformed_y = complex_block_position.im;

			//println!("{}", transformed_y);
			
            draw_rectangle(transformed_x, transformed_y, 1.0, 1.0, blockcolor);
			
        }
        
    }
	
/* 
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

		
	}*/
}


pub fn draw_world_color_only(chunks_in_view: &HashMap<IVec2, [BlockType; 1024]>){
    for chunkinfo in chunks_in_view{
        for index in 0..1024{
            let blockcolor:Color = match chunkinfo.1[index] {
                BlockType::Air => Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0},
                BlockType::Marvin => Color { r: 0.5, g: 0.4, b: 0.0, a: 1.0},
                BlockType::Dirt => Color { r: 0.5, g: 0.5, b: 0.1, a: 1.0},
                BlockType::Stone => Color { r: 0.4, g: 0.4, b: 0.45, a: 1.0},
                BlockType::Grass => Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0},
            };

            let x:i32 = index as i32%32;
            let y:i32 = index as i32/32;
            
            draw_rectangle((chunkinfo.0.x*32) as f32 + x as f32, (chunkinfo.0.y*32) as f32 + y as f32, 1.0, 1.0, blockcolor);
        }
        
    }
}