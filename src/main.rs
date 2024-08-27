use std::{cell::RefCell, collections::HashMap};


use chunk::{BlockType, ChunkWithOtherInfo, Planet};
use collision::{DynRect, MovableEntity};
use macroquad::{prelude::*, texture};
use num_complex::Complex;
use texturemanager::Texturemanager;

mod render;
mod collision;
mod chunk;
mod texturemanager;

//FUCKFUCKFUCK I HAVE TO LEARN Rc FUCK RC(9999X) WEAK PLS I BEG YOU, 
//OKOKOKOK i can skip many steps hopefully by not referencing the planet directly but a list they are in
// RefCell IS THE GOAT, THE GOAT

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Items {
	DirtBlock{amount: u32},
	StoneBlock{amount: u32},
	GrassBlock{amount: u32},
	PickAxe,
}

enum PlaceOrBreak{
	Place,
	Break,
	Wtf,
}

struct DroppedItem<'a>{
	entity: collision::MovableEntity<'a>,
	items: Items,
}


#[macroquad::main("Torus")]
async fn main() {
	println!("Hello, universe!");

	let terra = Planet{
		name: "Terra",
		space_position: RefCell::new(Vec2{x: 0.0, y: 0.0}),
		size: UVec2 { x: 20, y: 20}, 
		rotation: RefCell::new(0.0),
	};
	

	let mut player:collision::MovableEntity = collision::MovableEntity{
		dynrect: collision::DynRect{rect:Rect{x:1.0, y: 620.0, w: 1.75, h:2.6}, velocity: Vec2::ZERO},
		planet: Some(&terra),
	};

    let compacta_font = load_ttf_font("Assets/compacta.ttf").await.unwrap();

	let texturemanager: texturemanager::Texturemanager = texturemanager::texture_manager().await;

    let mut zoom:f32 = 48.0;
    let mut camera_rotation:f32 = 0.0;
    let mut camera_zoom = Vec2{x:1./10.0, y:1./10.0};
    let mut camera_target:Vec2 = Vec2 { x: 0.0, y: 0.0 };

	let drop:DroppedItem = DroppedItem{ 
		entity: collision::MovableEntity{
			dynrect: collision::DynRect{rect:Rect{x:1.4, y: 625.0, w: 1.7, h:1.7}, velocity: Vec2::ZERO},
			planet: Some(&terra),
		},
		items:Items::PickAxe,
	};

	let mut chunks_in_view:HashMap<IVec2,ChunkWithOtherInfo> = HashMap::new();
	let mut dropped_items:Vec<DroppedItem> = vec![drop];

	let mut player_hotbar:[Option<Items>;10] = [None; 10];
	let mut select_hotbar:i32 = 1;


	player_hotbar[0] = Some(Items::PickAxe);
	player_hotbar[1] = Some(Items::DirtBlock { amount: 1 });
	player_hotbar[9] = Some(Items::DirtBlock { amount: 1 });
    loop{
		let delta = get_frame_time();
    	clear_background(BLACK);
		movement_input(&mut player.dynrect, &delta, &mut zoom);
		collision::dynamic_rectangle_vs_planet_chunks(&delta, &mut player.dynrect, &chunks_in_view, &player.planet.unwrap());
		playermovement(&mut player.dynrect, &delta);
	
		pick_up_items(&player, &mut player_hotbar, &mut dropped_items);

		match keyboard_number() {
			Some(number) => select_hotbar = number as i32,
			None =>{}
		};

		*terra.rotation.borrow_mut() += 0.01;



    	camera_zoom *= zoom;
		camera_target = player.dynrect.rect.center();
    	let mut camera = Camera2D {
        	zoom: camera_zoom,
        	target: camera_target,
			rotation: camera_rotation,
        	..Default::default()
    	};
		
		
		

		chunk::chunks_in_view_manager(&camera, &mut chunks_in_view, player.planet);

		

		set_camera_target_to_position_planet(player.dynrect.rect.center(), &player.planet.unwrap(), &mut camera.target, &mut camera_zoom, &mut camera_rotation);
		set_camera(&camera);


		hotbar_logic(&camera, &player.planet.unwrap(), &mut chunks_in_view, &player_hotbar, &select_hotbar);

		//make it so that i only render the world the player is on, The situation in where he can see 2 planets at the same time should never happen
		//something like this render_world(player.planet), shit also need to add a point in which to see
		render::render_planet_chunks(&player.planet.unwrap(), &player.dynrect.rect.center(),&chunks_in_view, &texturemanager);
		

		render_dropped_items(&dropped_items, &texturemanager);
		render_entity(&player.planet.unwrap(), &player, &texturemanager.imposter);
		
		set_default_camera();
		render_hotbar(&player_hotbar, &texturemanager, &select_hotbar);
    	draw_fps(&compacta_font);
		
    	next_frame().await
    }
}

fn pick_up_items(player: &collision::MovableEntity, hotebaru: &mut [Option<Items>; 10], dropped_items: &mut Vec<DroppedItem>){
	let mut items_to_remove: Vec<usize> = Vec::new();

	for (iter, dropped_item) in dropped_items.iter().enumerate(){
		if dropped_item.entity.planet != player.planet{continue;}

		let distance_between = player.dynrect.rect.point().distance(dropped_item.entity.dynrect.rect.point());
		if distance_between >= 3.0{continue;}
		
		items_to_remove.push(iter);
		for (iteriter, bar) in hotebaru.iter().enumerate(){
			if bar.is_some() {continue};
			hotebaru[iteriter] = Some(dropped_item.items);

			break;
		}
	}
	for &index in items_to_remove.iter().rev() {
        dropped_items.remove(index);
    }
}


fn render_dropped_items(dropped_items: &Vec<DroppedItem>, texturemanager: &Texturemanager){


	for dropped_item in dropped_items.iter(){

		let texture:&Texture2D = match dropped_item.items {
			Items::DirtBlock { amount } => {&texturemanager.dirt},
			Items::GrassBlock { amount } => {&texturemanager.grass},
			Items::PickAxe => {&texturemanager.pickaxe},
			Items::StoneBlock { amount } => {&texturemanager.stone},
			_ => {&texturemanager.imposter},
		};
		render_entity(dropped_item.entity.planet.unwrap(), &dropped_item.entity, texture);
	}
}


fn keyboard_number() -> Option<u8>{
	if is_key_pressed(KeyCode::Key1) { return Some(0);}
	if is_key_pressed(KeyCode::Key2) { return Some(1);}
	if is_key_pressed(KeyCode::Key3) { return Some(2);}
	if is_key_pressed(KeyCode::Key4) { return Some(3);}
	if is_key_pressed(KeyCode::Key5) { return Some(4);}
	if is_key_pressed(KeyCode::Key6) { return Some(5);}
	if is_key_pressed(KeyCode::Key7) { return Some(6);}
	if is_key_pressed(KeyCode::Key8) { return Some(7);}
	if is_key_pressed(KeyCode::Key9) { return Some(8);}
	if is_key_pressed(KeyCode::Key0) { return Some(9);}
	return None;
}



fn hotbar_logic(camera: &Camera2D, planet: &Planet, chunks_in_view: &mut HashMap<IVec2,ChunkWithOtherInfo>, hotebaru: &[Option<Items>;10], select_hotbar:&i32){
	let item = hotebaru[*select_hotbar as usize];

	let item = match item {
		Some(x) => x,
		None => return,
	};



	if is_mouse_button_down(MouseButton::Left) {
		match item {
			Items::DirtBlock { amount } => place_block(&camera, &planet, chunks_in_view),
			Items::StoneBlock { amount } => todo!(),
			Items::GrassBlock { amount } => todo!(),
			Items::PickAxe => destroy_block(&camera, &planet, chunks_in_view),
			
		};
	}

}


fn place_block(camera: &Camera2D, planet: &Planet, chunks_in_view: &mut HashMap<IVec2,ChunkWithOtherInfo>){
	let camamara = camera.screen_to_world(mouse_position().into());

	let mut cemera:Vec2 = inverse_disk_position(camamara, &planet) + 0.5;

	

	if cemera.x <= 0.0 {cemera.x -= 1.0}

	let cemera:IVec2 = IVec2 { x: cemera.x as i32, y: cemera.y as i32 };

	let chunk_x: i32 = cemera.x.rem_euclid(planet.size.x as i32 * 32).div_euclid(32);
	let chunk_y: i32 = cemera.y.div_euclid(32);
	//println!("chunk: {}, mouse: {}", cemera.x, cemera.y);
	let chunktoread: Option<&mut ChunkWithOtherInfo> = chunks_in_view.get_mut(&IVec2 { x: chunk_x, y: chunk_y });

	let chunktoread: &mut ChunkWithOtherInfo = match chunktoread {
		Some(chunk) => chunk,
		None => {
			eprintln!(
				"trying to place at something that dont exist {} {}",
				chunk_x, chunk_y
			);
			return;
		}
	};
	
	let blockindex: usize = (cemera.x.rem_euclid(32) + (cemera.y.rem_euclid(32)) * 32) as usize;
	chunktoread.chunk[blockindex] = BlockType::Grass;

}

fn destroy_block(camera: &Camera2D, planet: &Planet, chunks_in_view: &mut HashMap<IVec2,ChunkWithOtherInfo>){
	let camamara = camera.screen_to_world(mouse_position().into());

	let mut cemera:Vec2 = inverse_disk_position(camamara, &planet) + 0.5;

	

	if cemera.x <= 0.0 {cemera.x -= 1.0}

	let cemera:IVec2 = IVec2 { x: cemera.x as i32, y: cemera.y as i32 };

	let chunk_x: i32 = cemera.x.rem_euclid(planet.size.x as i32 * 32).div_euclid(32);
	let chunk_y: i32 = cemera.y.div_euclid(32);
	//println!("chunk: {}, mouse: {}", cemera.x, cemera.y);
	let chunktoread: Option<&mut ChunkWithOtherInfo> = chunks_in_view.get_mut(&IVec2 { x: chunk_x, y: chunk_y });

	let chunktoread: &mut ChunkWithOtherInfo = match chunktoread {
		Some(chunk) => chunk,
		None => {
			eprintln!(
				"trying to destroy at something that dont exist {} {}",
				chunk_x, chunk_y
			);
			return;
		}
	};
	
	let blockindex: usize = (cemera.x.rem_euclid(32) + (cemera.y.rem_euclid(32)) * 32) as usize;
	chunktoread.chunk[blockindex] = BlockType::Air;

}


fn set_camera_target_to_position_planet(position: Vec2, planet: &Planet, camera_pos: &mut Vec2, camera_zoom: &mut Vec2, camera_rotation: &mut f32){
	let normalisedplayerx = ((position.x - 0.5)  *2.0 /(planet.size.x*32) as f32 -1.0) * std::f32::consts::PI;
	let normalisedplayery = ((position.y - 0.5) - (planet.size.y*32) as f32) *(std::f32::consts::TAU/(planet.size.x*32) as f32);

	let mut playercomplex = Complex{re:normalisedplayery + 10.0, im:normalisedplayerx + *planet.rotation.borrow()};

	
	playercomplex = Complex::exp(playercomplex);

	camera_pos.x = playercomplex.re;
	camera_pos.y = playercomplex.im;

	let zoom = f32::sqrt(f32::powf(playercomplex.re,2.)+f32::powf(playercomplex.im,2.)) *(std::f32::consts::TAU/(planet.size.x*32) as f32);

	camera_zoom.y = (1.0/screen_height())/zoom;
	camera_zoom.x = (1.0/screen_width())/zoom;

	
	*camera_rotation = (camera_pos.x.atan2(camera_pos.y) * (360./std::f32::consts::TAU)) + 180.;
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
	texture: &Texture2D,
){ // 99% will need to be more fixed later on
	//Def its own function, should be adapted for any entity

	// This should maybe be put in its own function, could alse be used as a base for render_world() unsure
	let normalisedplayerx = ((((entity.dynrect.rect.center().x - 0.5) *2.0) /(planet.size.x*32) as f32) -1.0) * std::f32::consts::PI;
	let normalisedplayery = ((entity.dynrect.rect.center().y - 0.5) - (planet.size.y*32) as f32) *(std::f32::consts::TAU/(planet.size.x*32) as f32);

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
		texture,
		player_node_x - player_size * (entity.dynrect.rect.w/2.0),
		player_node_y - player_size * (entity.dynrect.rect.h/2.0),
		WHITE,
		DrawTextureParams {
			dest_size: Some(vec2(player_size * entity.dynrect.rect.w,player_size *entity.dynrect.rect.h)),
			rotation: player_node_y.atan2(player_node_x)+std::f32::consts::FRAC_PI_2,
			..Default::default()
	}
);
    
}

fn inverse_disk_position(vec: Vec2, planet: &Planet) -> Vec2{

	let complex = Complex{re: vec.y, im: vec.x};

    let reversed = complex.ln();

    let normalisedy = reversed.re - 10.0;
    let normalisedx = reversed.im + *planet.rotation.borrow() - std::f32::consts::FRAC_PI_2;
	
    let position_x = ((normalisedx / std::f32::consts::PI) + 1.0) * (planet.size.x * 32)as f32 / 2.0;
    let position_y = (normalisedy / (std::f32::consts::TAU / (planet.size.x * 32)as f32)) + (planet.size.y * 32)as f32;
	Vec2{x: position_x * -1.0, y: position_y}
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

fn movement_input(player: &mut DynRect, delta: &f32, zoom: &mut f32){
	if is_key_down(KeyCode::A) {
		player.velocity.x -= 100.0 * delta;
	}
	if is_key_down(KeyCode::D) {
		player.velocity.x += 100.0 * delta;
	}
	if is_key_down(KeyCode::W) {
		player.velocity.y += 40.0 * delta;
	}
	if is_key_down(KeyCode::S) {
		player.velocity.y -= 40.0 * delta;
	}
	if is_key_down(KeyCode::KpAdd) {
		*zoom -= 4.0 * delta;
	}
	if is_key_down(KeyCode::KpSubtract) {
		*zoom += 4.0 * delta;
	}
}


fn render_hotbar(hotebaru: &[Option<Items>;10], texturemanager: &Texturemanager, select_hotbar:&i32){
	let scale:f32 = 4.0;

	let dynamic_x_offset = scale*2.0;
	let dynamic_y_offset = scale*2.0;

	draw_texture_ex(&texturemanager.hotbar,
		dynamic_x_offset,
		dynamic_y_offset,
		   WHITE,
		    DrawTextureParams{
				dest_size: Some(vec2(texturemanager.hotbar.width(), texturemanager.hotbar.height()) * scale),
				..Default::default()});

	for (iter, item) in hotebaru.iter().enumerate(){
		let item = match item {
			Some(x) => x,
			None => continue
		};

		let item_texture: &Texture2D  = match item {
			Items::DirtBlock { amount } => &texturemanager.dirt,
			Items::StoneBlock { amount } => todo!(),
			Items::GrassBlock { amount } => todo!(),
			Items::PickAxe =>  &texturemanager.pickaxe,
		};

		draw_texture_ex(item_texture,
			dynamic_x_offset - (item_texture.width() * scale) / 2.0 + ((iter as f32) * 18.0 + 10.0) * scale,
			dynamic_y_offset + (item_texture.height() * scale * 0.5) - (6.0 * scale),
			  WHITE,
			   DrawTextureParams{
				   dest_size: Some(vec2(16.0, 16.0) * scale),
				   ..Default::default()}
		);
	}
	draw_rectangle_lines(
		dynamic_x_offset - (22.0 * scale) / 2.0 + ((*select_hotbar as f32) * 18.0 + 10.0) * scale,
		dynamic_y_offset + (10.0 * scale * 0.5) - (6.0 * scale),
		22.0 * scale,
		22.0 * scale,
		2.0 * scale,
		BLUE,);
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