use std::collections::HashMap;

use macroquad::{
	color::{Color, WHITE}, math::{vec2, IVec2, UVec2, Vec2}, shapes::{draw_rectangle, draw_rectangle_ex, DrawRectangleParams}, texture::{draw_texture_ex, 
		DrawTextureParams,
		}
	};





use crate::{chunk::Planet, full_info_planet_to_space_coords};



//fix
pub fn render_planet_chunks(
	planet : &Planet,
	chunks_in_view: &HashMap<IVec2, crate::chunk::ChunkWithOtherInfo>,
	itemtypes: &Vec<crate::ItemType>,
){

	for chunkinfo in chunks_in_view{

        for index in 0..1024{

			if let Some(texture) = &itemtypes[chunkinfo.1.chunk[index] as usize].texture{

            let chunk_x:i32 = index as i32%32;
            let chunk_y:i32 = index as i32/32;

			let x = (chunkinfo.0.x*32) as f32 + chunk_x as f32;
			let y = (chunkinfo.0.y*32) as f32 + chunk_y as f32;

			//now comes the funny part(turning it into a disk) AUTOMATED LESFGO
			//my brain hurts
			let transformed = full_info_planet_to_space_coords(planet, &Vec2{x:x + 0.5, y:y + 0.5});


			draw_texture_ex(
				&texture,
				transformed.0.x - transformed.1/2.,
				transformed.0.y - transformed.1/2.,
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(transformed.1,transformed.1)),
					rotation: transformed.2,
					..Default::default()
				},
			);

			if true {continue;}
			if chunk_x == 0 || chunk_x == 31 || chunk_y == 0 || chunk_y == 31{
				draw_rectangle_ex(
					transformed.0.x - transformed.1/2.,
					transformed.0.y - transformed.1/2.,
					transformed.1,
					transformed.1,
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
}


pub fn _draw_world_color_only(chunks_in_view: &HashMap<UVec2, [usize; 1024]>){
    for chunkinfo in chunks_in_view{
        for index in 0..1024{
            let blockcolor:Color = match chunkinfo.1[index] {
                0 => Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0},
                1 => Color { r: 0.5, g: 0.4, b: 0.0, a: 1.0},
                2 => Color { r: 0.5, g: 0.5, b: 0.1, a: 1.0},
                3 => Color { r: 0.4, g: 0.4, b: 0.45, a: 1.0},
                4 => Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0},
				_ =>  Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0},
            };

            let x:i32 = index as i32%32;
            let y:i32 = index as i32/32;
            
            draw_rectangle((chunkinfo.0.x*32) as f32 + x as f32, (chunkinfo.0.y*32) as f32 + y as f32, 1.0, 1.0, blockcolor);
        }
        
    }
}
