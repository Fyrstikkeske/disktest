use macroquad::prelude::*;
use num_complex::Complex;


const ARRX:usize = 3;
const ARRY:usize = 10000;
//const WORLD_SIZE:f32 = 100.0;


#[macroquad::main("Torus")]
async fn main() {
	let mut playerx = 0.0;
	let mut playery = 1.3;
	let mut flydown = true;
	let mut WORLD_SIZE:f32 = 100.0;

    println!("Hello, world!");

    let mut array:[[Vec2;ARRY];ARRX] = [[Vec2{x:0.0,y: 0.0};ARRY];ARRX];

    for x in 0..ARRX{
    	for y in 0..ARRY{
    		array[x][y] = Vec2{
				x:(x as f32 *2.0 /ARRX as f32 -1.0) * std::f32::consts::PI,
				y: (y as f32 - ARRY as f32)/5.0 //y as f32 *2.0 /ARRY as f32 -1.0
			};
    	}
    }

	println!("{}", array[0][0]);

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
		let mut playercomplex = Complex{re:playery, im:playerx};
		playercomplex = Complex::exp(playercomplex);
		let player_node_x = playercomplex.re * 100.;
		let player_node_y = playercomplex.im * 100.;
		let player_size = f32::sqrt(f32::powf(playercomplex.re,2.)+f32::powf(playercomplex.im,2.))*(100.0*0.05);

    	
    	for x in 0..ARRX{
    		for y in 0..ARRY{
    			let mut complex = Complex{re:array[x][y].y, im:array[x][y].x};
    			complex = Complex::exp(complex);
				let node_x = complex.re * WORLD_SIZE;
				let node_y = complex.im * WORLD_SIZE;
				let size = f32::sqrt(f32::powf(complex.re,2.)+f32::powf(complex.im,2.))*(WORLD_SIZE*0.08);


				
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
		draw_circle(player_node_x - player_size/2. + 500.0,  player_node_y - player_size/2. + 500.0, player_size, Color{r:1., g:1. , b:1., a:1.0});
		playerx +=0.1;
		if flydown == true {playery -=0.01;}
		else{playery +=0.01;}

		if playery < -1.0 {flydown = false}
		if playery > 1.0 {flydown = true}

		WORLD_SIZE += WORLD_SIZE/10.;

    	next_frame().await
    }
}


