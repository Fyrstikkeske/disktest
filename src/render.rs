use std::collections::HashMap;

use macroquad::{
	color::{Color, WHITE}, math::{vec2, IVec2, UVec2, Vec2}, shapes::{draw_rectangle, draw_rectangle_ex, DrawRectangleParams}, texture::{draw_texture_ex, 
		DrawTextureParams,
		Texture2D,
		}
	};



use num_complex::Complex;

use crate::chunk::{BlockType, Planet};



//fix
pub fn render_planet_chunks(
	planet : &Planet,
	chunks_in_view: &HashMap<IVec2, crate::chunk::ChunkWithOtherInfo>,
	texturemanager: &crate::texturemanager::Texturemanager,
){

	for chunkinfo in chunks_in_view{

        for index in 0..1024{

			let texture_to_use = match chunkinfo.1.chunk[index] {
				BlockType::Dirt => &texturemanager.dirt,
				BlockType::Grass => &texturemanager.grass,
				BlockType::Stone => &texturemanager.stone,
				_ => {continue;}
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
			let pre_complex_block_position = Complex{re:normalised_block_position.y, im:normalised_block_position.x + *planet.rotation.borrow()};
			let complex_block_position = Complex::exp(pre_complex_block_position);
			let transformed_x = complex_block_position.re + planet.space_position.borrow().x;
			let transformed_y = complex_block_position.im + planet.space_position.borrow().y;

			let size = f32::sqrt(f32::powf(complex_block_position.re,2.) + f32::powf(complex_block_position.im,2.)) *((std::f32::consts::TAU)/(planet.size.x*32) as f32);

			draw_texture_ex(
				texture_to_use,
				transformed_x - size/2.,
				transformed_y - size/2.,
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(size,size)),
					rotation: complex_block_position.im.atan2(complex_block_position.re) +std::f32::consts::PI/2.,
					..Default::default()
				},
			);

			if false {continue;}
			if chunk_x == 0 || chunk_x == 31 || chunk_y == 0 || chunk_y == 31{
				draw_rectangle_ex(
					transformed_x - size/2.,
					transformed_y - size/2.,
					size,
					size,
					DrawRectangleParams {
						color: WHITE,
						//rotation: transformed_y.atan2(transformed_x) +std::f32::consts::PI/2.,
						..Default::default()
					},
				);
			}
        }
    }
}


pub fn draw_world_color_only(chunks_in_view: &HashMap<UVec2, [BlockType; 1024]>){
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
