use std::{cell::RefCell, collections::HashMap};


use chunk::Planet;
use collision::DynRect;
use macroquad::prelude::*;
use num_complex::Complex;
use render::Texturemanager;
use scene::camera_pos;

mod render;
mod collision;
mod chunk;


//FUCKFUCKFUCK I HAVE TO LEARN Rc FUCK RC(9999X) WEAK PLS I BEG YOU, 
//OKOKOKOK i can skip many steps hopefully by not referencing the planet directly but a list they are in
// RefCell IS THE GOAT, THE GOAT


#[macroquad::main("Torus")]
async fn main() {
	println!("Hello, universe!");

	let terra = Planet{
		name: "Terra",
		space_position: RefCell::new(Vec2{x: 0.0, y: 0.0}),
		size: UVec2 { x: 100, y: 60 }, 
		rotation: RefCell::new(3.4),
	};
	

	let mut player:collision::MovableEntity = collision::MovableEntity{
		dynrect: collision::DynRect{rect:Rect{x:1.0, y: 100.0, w: 1.0, h:1.0}, velocity: Vec2::ZERO},
		planet: Some(&terra),
	};

    let compacta_font = load_ttf_font("Assets/compacta.ttf").await.unwrap();
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


    let mut zoom:f32 = 32.0;
    
    let mut camera_zoom = Vec2{x:1./10.0, y:1./10.0};
    let mut camera_target:Vec2 = Vec2 { x: 0.0, y: 0.0 };

	let mut chunks_in_view:HashMap<IVec2,[chunk::BlockType; chunk::CHUNKSIZE]> = HashMap::new();
	
    loop{
		let delta = get_frame_time();
    	clear_background(BLACK);
		movement_input(&mut player.dynrect, &delta);
		
		playermovement(&mut player.dynrect, &delta);

		//*terra.rotation.borrow_mut() += 0.01;

		camera_zoom.y = 1./screen_height();
    	camera_zoom.x = 1./screen_width();
		
    	camera_zoom *= zoom;
		camera_target = player.dynrect.rect.center();

    	let mut camera = Camera2D {
        	zoom: camera_zoom,
        	target: camera_target,
        	..Default::default()
    	};

		

		chunk::chunks_in_view_manager(&camera, &mut chunks_in_view, player.planet);
		collision::dynamic_rectangle_vs_planet_chunks(&delta, &mut player.dynrect, &chunks_in_view, &player.planet.unwrap());

		set_camera_target_to_position_planet(player.dynrect.rect.center(), &player.planet.unwrap(), &mut camera.target);
		set_camera(&camera);
		
		

		//make it so that i only render the world the player is on, The situation in where he can see 2 planets at the same time should never happen
		//something like this render_world(player.planet), shit also need to add a point in which to see
		render::render_planet_chunks(&player.planet.unwrap(), &player.dynrect.rect.center(),&chunks_in_view, &texturemanager);
		
		render_entity(&player.planet.unwrap(), &player, &texturemanager);


		set_default_camera();
    	draw_fps(&compacta_font);
		
    	next_frame().await
    }
}


fn set_camera_target_to_position_planet(position: Vec2, planet: &Planet, camera_pos: &mut Vec2){
	let normalisedplayerx = (position.x *2.0 /(planet.size.x*32) as f32 -1.0) * std::f32::consts::PI;
	let normalisedplayery = (position.y - (planet.size.y*32) as f32) *(std::f32::consts::TAU/(planet.size.x*32) as f32);

	let mut playercomplex = Complex{re:normalisedplayery + 10.0, im:normalisedplayerx + *planet.rotation.borrow()};

	
	playercomplex = Complex::exp(playercomplex);

	camera_pos.x = playercomplex.re;
	camera_pos.y = playercomplex.im;
}


fn playermovement(player: &mut DynRect, delta: &f32){
	player.rect.x = player.rect.x + (player.velocity.x * delta);
	player.rect.y = player.rect.y + (player.velocity.y * delta);
	player.velocity.x = player.velocity.x * 0.96;
	//player.velocity.y -= 9.81* delta;
	if player.velocity.x.abs() < 4.{player.velocity.x = player.velocity.x * 0.89;};
}



fn render_entity(
	planet: &Planet,
	entity: &collision::MovableEntity,
	texturemanager: &Texturemanager,
){ // 99% will need to be more fixed later on
	//Def its own function, should be adapted for any entity

	// This should maybe be put in its own function, could alse be used as a base for render_world() unsure
	let normalisedplayerx = (entity.dynrect.rect.x *2.0 /(planet.size.x*32) as f32 -1.0) * std::f32::consts::PI;
	let normalisedplayery = (entity.dynrect.rect.y - (planet.size.y*32) as f32) *(std::f32::consts::TAU/(planet.size.x*32) as f32);


	//this will be maybe a little more difficoult.
	//world_offset_height must equal a value that makes the entity be in the right y value

	let mut playercomplex = Complex{re:normalisedplayery + 10.0, im:normalisedplayerx + *planet.rotation.borrow()};
	

	playercomplex = Complex::exp(playercomplex);
	let player_node_x = playercomplex.re;
	let player_node_y = playercomplex.im;
	let player_size = f32::sqrt(f32::powf(playercomplex.re,2.)+f32::powf(playercomplex.im,2.)) *(std::f32::consts::TAU/(planet.size.x*32) as f32);
	//let player_size = 10.0;
	//println!("{}",playercomplex.re);
	//println!("{:?}", playercomplex);
	draw_texture_ex(
		&texturemanager.imposter,
		player_node_x - player_size/2.0,
		player_node_y - player_size/2.0,
		WHITE,
		DrawTextureParams {
			dest_size: Some(vec2(player_size,player_size)),
			rotation: player_node_y.atan2(player_node_x)+std::f32::consts::FRAC_PI_2,
			..Default::default()
	}
);
}

fn draw_fps(compacta_font:&Font){
    draw_text_ex(
        format!("{}", get_fps()).as_str(),
        20.0,
        30.0,
        TextParams {
            font_size: 30,
            font: Some(compacta_font),
            ..Default::default()}
        );

}

fn movement_input(player: &mut DynRect, delta: &f32){
	if is_key_down(KeyCode::A) {
		player.velocity.x -= 40.0 * delta;
	}

	if is_key_down(KeyCode::D) {
		player.velocity.x += 40.0 * delta;
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