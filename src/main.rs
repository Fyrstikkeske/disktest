use macroquad::prelude::*;
use num_complex::Complex;

const TWOPI:f32 = 6.2831853072;
const ARRX:usize = 1000;
const ARRY:usize = 100;
const WORLD_SIZE:f32 = 100.0;


#[macroquad::main("Torus")]
async fn main() {
    println!("Hello, world!");

    let mut array:[[Vec2;ARRY];ARRX] = [[Vec2{x:0.0,y: 0.0};ARRY];ARRX];

    for x in 0..ARRX{
    	for y in 0..ARRY{ //no idea why 2PI is in X, but it just works 
    		array[x][y] = Vec2{
				x:(x as f32/ARRX as f32) * TWOPI,
				y: y as f32/ ARRY as f32};
    	}
    }


	let stone = Texture2D::from_file_with_format(
	    include_bytes!("../textures/stone.png"),
	    None,
	);
	let dirt = Texture2D::from_file_with_format(
	    include_bytes!("../textures/dirt.png"),
	    None,
	);
	let grass = Texture2D::from_file_with_format(
	    include_bytes!("../textures/grass.png"),
	    None,
	);
	stone.set_filter(FilterMode::Nearest);
	dirt.set_filter(FilterMode::Nearest);
	grass.set_filter(FilterMode::Nearest);

    loop{
    	clear_background(BLACK);
		
    	
    	for x in 0..ARRX{
    		for y in 0..ARRY{
    			let mut complex = Complex{re:array[x][y].y, im:array[x][y].x};
    			complex = Complex::exp(complex);
				let node_x = complex.re * WORLD_SIZE;
				let node_y = complex.im * WORLD_SIZE;
				let size = 1.0;//f32::sqrt(f32::powf(node_x,2.)+f32::powf(node_y,2.));


				
				//not needed but here as why not in case 
				//draw_circle( node_x/50. + 500.0,  node_y/50. + 500.0, size, Color{r:0.1 * x as f32, g:0.1* y as f32, b:1.0, a:1.0});

				if y <= ARRY - 3{
					draw_texture_ex(
						&stone,
						node_x - size/2. + 500.0,
						node_y - size/2. + 500.0,
						WHITE,
						DrawTextureParams {
							dest_size: Some(vec2(size,size)),
							rotation: node_y.atan2(node_x),
							..Default::default()
						},
					);	
				}

				if y == ARRY - 2{
					draw_texture_ex(
						&dirt,
						node_x - size/2. + 500.0,
						node_y - size/2. + 500.0,
						WHITE,
						DrawTextureParams {
							dest_size: Some(vec2(size,size)),
							rotation: node_y.atan2(node_x),
							..Default::default()
						},
					);	
				}
				
				if y == ARRY - 1{
					draw_texture_ex(
						&grass,
						node_x - size/2. + 500.0,
						node_y - size/2. + 500.0,
						WHITE,
						DrawTextureParams {
							dest_size: Some(vec2(size,size)),
							rotation: node_y.atan2(node_x)+std::f32::consts::PI/2.,
							..Default::default()
						},
					);	
				}
				
    		}
    	}

    	next_frame().await
    }
}


