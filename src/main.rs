use macroquad::prelude::*;
use num_complex::Complex;


const ARRX:usize = 100;
const ARRY:usize = 100;



#[macroquad::main("Torus")]
async fn main() {
	let mut playerx = 0.0;
	let mut playery = 0.05;
	let mut world_offset_rotation:f32 = 0.0;
	let mut world_offset_height:f32 = 6.0;
	let mut world_offset_global_x:f32 = 960.0;
	let mut world_offset_global_y:f32 = 540.0;

    println!("Hello, world!");

    let mut box_world:[[Vec2;ARRY];ARRX] = [[Vec2{x:0.0,y: 0.0};ARRY];ARRX];

    for x in 0..ARRX{
    	for y in 0..ARRY{
    		box_world[x][y] = Vec2{
				x:(x as f32 *2.0 /ARRX as f32 -1.0) * std::f32::consts::PI,
				y: ((y+1) as f32 - ARRY as f32) *((std::f32::consts::PI*2.)/ARRX as f32)
			};
    	}
    }

	println!("{}", box_world[0][0]);

	let imposter = Texture2D::from_file_with_format(
		include_bytes!("../textures/imposter.png"),
		None,
	);
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

		if is_key_down(KeyCode::Right) {
            world_offset_rotation -= 0.05;
        }

		if is_key_down(KeyCode::Left) {
            world_offset_rotation += 0.05;
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

    	
    	for x in 0..ARRX{
    		for y in 0..ARRY{
    			let mut worlds_complex = Complex{re:box_world[x][y].y + world_offset_height, im:box_world[x][y].x + world_offset_rotation};
    			worlds_complex = Complex::exp(worlds_complex);
				let node_x = worlds_complex.re;
				let node_y = worlds_complex.im;
				let size = f32::sqrt(f32::powf(worlds_complex.re,2.)+f32::powf(worlds_complex.im,2.)) *((std::f32::consts::PI*2.)/ARRX as f32);

				
				
				//not needed but here as why not in case 
				//draw_circle( node_x/50. + 500.0,  node_y/50. + 500.0, size, Color{r:0.1 * x as f32, g:0.1* y as f32, b:1.0, a:1.0});

				if y <= ARRY - 3{
					draw_texture_ex(
						&stone,
						node_x - size/2. + world_offset_global_x,
						node_y - size/2. + world_offset_global_y,
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
						node_x - size/2. + world_offset_global_x,
						node_y - size/2. + world_offset_global_y,
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
						node_x - size/2. + world_offset_global_x,
						node_y - size/2. + world_offset_global_y,
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
		draw_texture_ex(
				&imposter,
				player_node_x - player_size/2. + world_offset_global_x,
				player_node_y - player_size/2. + world_offset_global_y,
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(player_size,player_size)),
					rotation: player_node_y.atan2(player_node_x)+std::f32::consts::PI/2.,
					..Default::default()
				}
			);
		//playerx +=0.1;
		//if flydown == true {playery -=0.01;}
		//else{playery +=0.01;}

		//if playery < -1.0 {flydown = false}
		//if playery > 1.0 {flydown = true}
		
    	next_frame().await
    }
}


