use core::f32;
use std::{cell::RefCell, collections::HashMap, rc::Rc};


use chunk::{ChunkWithOtherInfo, Planet};
use collision::{DynRect, MovableEntity};
use macroquad::prelude::*;
use num_complex::Complex;
use texturemanager::Texturemanager;

use crate::collision::dynamic_rectangle_vs_planet_chunks;

mod render;
mod collision;
mod chunk;
mod texturemanager;
//FUCKFUCKFUCK I HAVE TO LEARN Rc FUCK RC(9999X) WEAK PLS I BEG YOU, 
//OKOKOKOK i can skip many steps hopefully by not referencing the planet directly but a list they are in
// RefCell IS THE GOAT, THE GOAT

// need this or else i will do something evil.
struct GameState<'a>{
	planets: Vec<Rc<RefCell<Planet<'a>>>>,
	player: collision::MovableEntity<'a>,
	camera: Camera2D, 
	delta: f32,
	chunks_in_view: HashMap<IVec2,ChunkWithOtherInfo>,
	texturemanager: texturemanager::Texturemanager,
	player_inventory: [Option<Item>;10*5],
	select_hotbar: i32,
	spaceships: Vec<Rc<RefCell<SpaceShip<'a>>>>,
	dropped_items: Vec<DroppedItem<'a>>,
	is_inventory_open: bool,
	touches_floor: bool,
	itemtypes: Vec<ItemType>,
	item_references: HashMap<&'a str , usize>,
}

#[derive(Clone, Copy)]
pub struct Item {
    pub item_type_id: usize,
    pub amount: u32,
}

pub struct ItemType {
    pub id: String,
    pub name: String,
    pub texture: Option<Texture2D>,
    pub placeable: bool,
    pub max_stack: u32,
	pub tool: bool,
	pub block: bool,
	pub collidable: bool,
}

enum PlaceOrBreak{
	Place,
	Break,
	Wtf,
}
#[derive(Debug)]
enum SpaceShipsTypes{
	Sigma,
	Ohio,
	Simple,
	IUseALotOfSpaceships,
}


struct DroppedItem<'a>{
	entity: collision::MovableEntity<'a>,
	item: Item,
}
#[derive(Debug)]
struct SpaceShip<'a>{
	entity: collision::MovableEntity<'a>,
	fuel: i32,
	which: SpaceShipsTypes,
}

#[macroquad::main("Torus")]
async fn main() {
	println!("Hello, universe!");

	let terra:Rc<RefCell<Planet>> = Rc::new(RefCell::new(Planet{
		name: "Terra",
		space_position: RefCell::new(Vec2{x: 0.0, y: 0.0}),
		size: UVec2 { x: 20, y: 20}, 
		rotation: RefCell::new(f32::consts::PI),
	}));

	let planet_me:Rc<RefCell<Planet>> = Rc::new(RefCell::new(Planet{
		name: "PlanetMe",
		space_position: RefCell::new(Vec2{x: 0.0, y: 100.0}),
		size: UVec2 { x: 20, y: 10},
		rotation: RefCell::new(0.0),
	}));
	
	let player:collision::MovableEntity = collision::MovableEntity{
		dynrect:collision::DynRect{
			rect:Rect{x:1.0,y:(terra.borrow().size.y*32)as f32,w:1.75,h:2.6},
			velocity:Vec2::ZERO},
			planet:None,
			riding:None,
			rot: 0.0
		};

	

    let compacta_font = load_ttf_font("Assets/compacta.ttf").await.unwrap();

	let texturemanager: texturemanager::Texturemanager = texturemanager::texture_manager().await;
	/*
	let drop:DroppedItem = DroppedItem{
		entity: collision::MovableEntity{
			dynrect: collision::DynRect{rect:Rect{x:10.4, y: 605.0, w: 1.7, h:1.7}, velocity: Vec2::ZERO},
			planet: None,
			riding: None,
			rot: 0.0 
		},
		item:Items::PickAxe,
	}; */

	let sigma_spaceship:Rc<RefCell<SpaceShip>> =  Rc::new(RefCell::new(SpaceShip{
		entity: collision::MovableEntity{
			dynrect: collision::DynRect{rect:Rect{x:0.0, y: 620.0, w: 5.75, h:10.0}, velocity: Vec2::ZERO},
			planet: None,
			riding: None,
			rot: 0.0,
		},
	fuel: 100,
	which: SpaceShipsTypes::Simple,
	}));



	let mut gamestate: GameState = GameState{
		planets: Vec::new(),
		player: player,
		camera: Camera2D::default(),
		delta: 0.0,
		chunks_in_view: HashMap::new(),
		texturemanager: texturemanager::texture_manager().await,
		player_inventory: [const { None }; 10*5],
		select_hotbar: 1,
		spaceships: Vec::new(),
		dropped_items: Vec::new(),
		is_inventory_open:false,
		touches_floor: false,
		itemtypes: Vec::new(),
		item_references: HashMap::new(),
	};

	gamestate.itemtypes.push(
		ItemType{ id: "Air".to_string(),
			name: "DIRTMADAFAKA".to_string(),
			texture: None,
			placeable: false,
			max_stack: 64,
			tool: false,
			block: false,
			collidable: false,
		},
	);
	gamestate.item_references.insert("Air", gamestate.itemtypes.len() - 1);

	gamestate.itemtypes.push(
		ItemType{ id: "Dirt".to_string(),
			name: "DIRTMADAFAKA".to_string(),
			texture: Some(load_texture("textures/dirt.png").await.unwrap()),
			placeable: true,
			max_stack: 64,
			tool: false,
			block: false,
			collidable: true,
		},		
	);
	gamestate.item_references.insert("Dirt", gamestate.itemtypes.len() - 1);

	gamestate.itemtypes.push(
		ItemType{ id: "Stone".to_string(),
			name: "DIRTMADAFAKA".to_string(),
			texture: Some(load_texture("textures/stone.png").await.unwrap()),
			placeable: true,
			max_stack: 64,
			tool: false,
			block: false,
			collidable: true,
		},
	);
	gamestate.item_references.insert("Stone", gamestate.itemtypes.len() - 1);

	gamestate.itemtypes.push(
		ItemType{ id: "Grass".to_string(),
			name: "DIRTMADAFAKA".to_string(),
			texture: Some(load_texture("textures/grass.png").await.unwrap()),
			placeable: true,
			max_stack: 64,
			tool: false,
			block: false,
			collidable: true,
		},
	);
	gamestate.item_references.insert("Grass", gamestate.itemtypes.len() - 1);


	for i in gamestate.itemtypes.iter_mut(){
		if let Some(texture) = &i.texture {
			texture.set_filter(FilterMode::Nearest);
		}
	}
	
	/*
	gamestate.player_inventory[0] = Some(Items::PickAxe);
	gamestate.player_inventory[1] = Some(Items::DirtBlock { amount: 1 });
	gamestate.player_inventory[2] = Some(Items::StoneBlock { amount: 1 });
	gamestate.player_inventory[3] = Some(Items::GrassBlock { amount: 1 });
	gamestate.player_inventory[9] = Some(Items::DirtBlock { amount: 1 });
	gamestate.player_inventory[10] = Some(Items::PickAxe);*/


	gamestate.planets.push(terra);
	gamestate.planets.push(planet_me);
	gamestate.player.planet = Some(gamestate.planets[0].clone());
	gamestate.spaceships.push(sigma_spaceship);
	gamestate.spaceships[0].borrow_mut().entity.planet = Some(gamestate.planets[0].clone());

	gamestate.player_inventory[10] = Some(Item{ item_type_id: 0, amount: 2 });
	gamestate.dropped_items.push(
		DroppedItem{ entity: MovableEntity{ dynrect: DynRect{ rect: Rect { x: 3.0, y: 620.0, w: 1.0, h: 1.0}, velocity: Vec2 { x: 0.0, y: 0.0 } }, 
		planet: Some(gamestate.planets[0].clone()), riding: None, rot: 0.0 }, item: Item{ item_type_id: 0, amount: 1 } }
	);

	//THATS WHY HE IS THE GOAT!!! THE GOAT!!!!!!
    loop{
		gamestate.delta = get_frame_time();
    	clear_background(BLACK);

		
		planets_system(&mut gamestate);



		camera_manager_because_fucking_everything_is_broken(&mut gamestate.player, &mut gamestate.camera);
		gamestate.camera.zoom *= 48.;
		set_camera(&gamestate.camera);

		if gamestate.player.planet.is_some(){
			on_planet(&mut gamestate);
		}else {
			off_planet(&mut gamestate)
		}
		


		spaceship_system(&mut gamestate);
		
		pick_up_items(&gamestate.player, &mut gamestate.player_inventory, &mut gamestate.dropped_items, &gamestate.delta);
		if let Some(number) = keyboard_number() {
			gamestate.select_hotbar = number as i32 
		}

		if is_key_pressed(KeyCode::Escape) { gamestate.is_inventory_open = !gamestate.is_inventory_open};



		render_dropped_items(&gamestate.dropped_items, &gamestate.itemtypes );


		set_default_camera();


		render_hotbar(&gamestate.player_inventory, &texturemanager, &gamestate.select_hotbar, &gamestate.itemtypes);
		if gamestate.is_inventory_open{
			render_rest_of_the_inventory(&gamestate.player_inventory, &texturemanager, &gamestate.itemtypes);
		}
		if false {
			draw_text_debug(&compacta_font, &gamestate);
		}
    	
		name_plate(&compacta_font, &gamestate);
		gamestate.camera = Camera2D::default();
    	next_frame().await
    }
}

fn normalise_stuff_on_planets(gamestate: &mut GameState){
	gamestate.player.dynrect.rect.x = gamestate.player.dynrect.rect.x.rem_euclid((gamestate.player.planet.clone().unwrap().borrow().size.x * 32) as f32);
}

fn spaceship_system(gamestate:&mut GameState){
	if gamestate.player.planet.is_some() && is_key_pressed(KeyCode::Enter){
		if gamestate.player.riding.is_some() {
			gamestate.player.riding = None;
		}else {
			for spaceship in &gamestate.spaceships{
				if spaceship.borrow().entity.planet.is_none(){ continue;}
				let distance = gamestate.player.dynrect.rect.center().distance(spaceship.borrow().entity.dynrect.rect.center());
				let touches = collision::loopingaabb(&gamestate.player.dynrect.rect, &spaceship.borrow().entity.dynrect.rect,(gamestate.player.planet.clone().unwrap().borrow_mut().size.x * 32) as f32, 100000.0);
				if distance < 5.0 || touches{gamestate.player.riding = Some(spaceship.clone())}
			}
		}
	}

	if let Some(spaceship) = &gamestate.player.riding{
		rocket_input(&mut spaceship.borrow_mut().entity, &gamestate.delta)
	}

	for spaceship in &mut gamestate.spaceships{
		let mut spaceship = spaceship.borrow_mut(); 

		if let Some(planet) = spaceship.entity.planet.clone() {
			render_entity(&planet.borrow(), &spaceship.entity, &gamestate.texturemanager.simple_spaceship);

			if spaceship.entity.dynrect.rect.y > (spaceship.entity.planet.clone().unwrap().borrow().size.y * 32 + 16) as f32 {
				let center = spaceship.entity.dynrect.rect.center(); // Immutable borrow ends here
				let new_x = full_info_planet_to_space_coords(&planet.borrow(), &center).0 - spaceship.entity.dynrect.rect.size() / 2.0;
				spaceship.entity.dynrect.rect.move_to(new_x);
				spaceship.entity.planet = None;

				let oldvelocity = spaceship.entity.dynrect.velocity * 0.1;
				let transformed = full_info_planet_to_space_coords(&planet.borrow(), &center);

				spaceship.entity.dynrect.velocity.y = oldvelocity.x * -transformed.2.sin();
				spaceship.entity.dynrect.velocity.y += oldvelocity.y * transformed.2.cos();

				spaceship.entity.dynrect.velocity.x = oldvelocity.x * transformed.2.cos();
				spaceship.entity.dynrect.velocity.x += oldvelocity.y * transformed.2.sin();


				spaceship.entity.rot += transformed.2;
				return;
			}

			collision::dynamic_rectangle_vs_planet_chunks(
				&gamestate.delta,
				&mut spaceship.entity.dynrect,
				&gamestate.chunks_in_view,
				&planet.borrow(),
			&gamestate.itemtypes);
			spaceship.entity.dynrect.rect.x += spaceship.entity.dynrect.velocity.x * gamestate.delta;
			spaceship.entity.dynrect.rect.y += spaceship.entity.dynrect.velocity.y * gamestate.delta;
			spaceship.entity.dynrect.rect.x = spaceship.entity.dynrect.rect.x.rem_euclid((spaceship.entity.planet.clone().unwrap().borrow().size.x * 32) as f32);
		}else {
			for planet in gamestate.planets.clone(){
				let mut position_if_rocket_was_on_planet:Vec2 = inverse_disk_position(spaceship.entity.dynrect.rect.center(), &planet.borrow());

				if position_if_rocket_was_on_planet.x <= 0.0 {position_if_rocket_was_on_planet.x -= 1.0}
			
				if position_if_rocket_was_on_planet.y as u32 <= &planet.borrow().size.y * 32{
					spaceship.entity.planet = Some(planet.clone());
					spaceship.entity.dynrect.rect.move_to(position_if_rocket_was_on_planet);


					let oldvelocity = spaceship.entity.dynrect.velocity * 10.0;
					
					let transformed = full_info_planet_to_space_coords(&planet.borrow(), &position_if_rocket_was_on_planet);

					spaceship.entity.dynrect.velocity.y = oldvelocity.x * (transformed.2).sin();
					spaceship.entity.dynrect.velocity.y += oldvelocity.y * (transformed.2).cos();

					spaceship.entity.dynrect.velocity.x = oldvelocity.x * (-transformed.2).cos();
					spaceship.entity.dynrect.velocity.x += oldvelocity.y * (-transformed.2).sin();


					spaceship.entity.rot -= transformed.2;
					return;
				}
			}

			render_entity_space(&spaceship.entity, &gamestate.texturemanager.simple_spaceship);
			spaceship.entity.dynrect.rect.x += spaceship.entity.dynrect.velocity.x * gamestate.delta;
			spaceship.entity.dynrect.rect.y -= spaceship.entity.dynrect.velocity.y * gamestate.delta;
		}
	}
}

fn planets_system(gamestate:&mut GameState){
	for planet in &gamestate.planets {
        let planet = planet.borrow_mut();
        let mut rotation_mut = planet.rotation.borrow_mut();
		*rotation_mut += gamestate.delta * 0.5;
        *rotation_mut = rotation_mut.rem_euclid(-std::f32::consts::TAU);
    }
}

fn off_planet(gamestate:&mut GameState){
	if let Some(spaceship) = &gamestate.player.riding{
		gamestate.player.dynrect.rect.move_to(spaceship.borrow().entity.dynrect.rect.center() - gamestate.player.dynrect.rect.size()/2.0);
	}else {
		movement_input_space(&mut gamestate.player.dynrect, &gamestate.delta);
		playermovementspace(&mut gamestate.player.dynrect, &gamestate.delta);
	}




	for planet in gamestate.planets.clone(){
		let mut playerposition_if_on_planet:Vec2 = inverse_disk_position(gamestate.player.dynrect.rect.center(), &planet.borrow());

		if playerposition_if_on_planet.x <= 0.0 {playerposition_if_on_planet.x -= 1.0}
	
		if playerposition_if_on_planet.y as u32 <= &planet.borrow().size.y * 32{
			gamestate.player.planet = Some(planet.clone());
			gamestate.player.dynrect.rect.move_to(playerposition_if_on_planet);
			return;
		}
		

		let rect:Rect = Rect{
			x: playerposition_if_on_planet.x,
			y: playerposition_if_on_planet.y,
			w: 4.0,
			h: 6.0,
		};

		chunk::chunks_in_view_manager(&rect, &mut gamestate.chunks_in_view, &planet.borrow(), &gamestate.itemtypes, &gamestate.item_references);
		render::render_planet_chunks(&planet.borrow(),&gamestate.chunks_in_view, &gamestate.itemtypes);
	}

	if gamestate.player.riding.is_none(){
		render_entity_space(&gamestate.player, &gamestate.texturemanager.imposter);
	}
}

fn on_planet(gamestate:&mut GameState){
	//*gamestate.player.planet.clone().unwrap().borrow_mut().rotation.borrow_mut() += 0.001;
	//gamestate.planets[0].borrow_mut().space_position.borrow_mut().x -= 0.001;
	
	if gamestate.player.dynrect.rect.y > (gamestate.player.planet.clone().unwrap().borrow().size.y * 32 + 16) as f32 {
		chunk::save_planet_from_manager(&mut gamestate.chunks_in_view, &gamestate.player.planet.clone().unwrap().borrow(), &gamestate.itemtypes);
		gamestate.player.dynrect.rect.move_to(full_info_planet_to_space_coords(&gamestate.player.planet.clone().unwrap().borrow(), &gamestate.player.dynrect.rect.center()).0 - gamestate.player.dynrect.rect.size()/2.0);
		gamestate.player.planet = None;
		gamestate.player.dynrect.velocity = Vec2::ZERO;
		gamestate.camera.rotation = 0.0;
		return;
	}
	
	if let Some(spaceship) = &gamestate.player.riding{
		gamestate.player.dynrect.rect.move_to(spaceship.borrow().entity.dynrect.rect.center() - gamestate.player.dynrect.rect.size()/2.0);
	}else {
		if gamestate.touches_floor{
			if is_key_down(KeyCode::Space) {
				gamestate.player.dynrect.velocity.y = 2000.0 * gamestate.delta;
			}
			if is_key_down(KeyCode::A) {
				gamestate.player.dynrect.velocity.x -= 100.0 * gamestate.delta;
			}
			if is_key_down(KeyCode::D) {
				gamestate.player.dynrect.velocity.x += 100.0 * gamestate.delta;
			}
		} else {
			if is_key_down(KeyCode::A) {
				gamestate.player.dynrect.velocity.x -= 40.0 * gamestate.delta;
			}
			if is_key_down(KeyCode::D) {
				gamestate.player.dynrect.velocity.x += 40.0 * gamestate.delta;
			}
		}
		let info: collision::RayRectInfo = dynamic_rectangle_vs_planet_chunks(
			&gamestate.delta,
			&mut gamestate.player.dynrect,
			&gamestate.chunks_in_view,
			&gamestate.player.planet.clone().unwrap().borrow(),
		&gamestate.itemtypes);
		if info.contact_normal.y > 0.0{
			gamestate.touches_floor = true
		}else {
			gamestate.touches_floor = false
		}
		playermovement(&mut gamestate.player.dynrect, &gamestate.delta);
	}


	let rect:Rect = Rect{
		x: gamestate.player.dynrect.rect.center().x,
    	y: gamestate.player.dynrect.rect.center().y,
    	w: 4.0,
    	h: 6.0,
	};
	chunk::chunks_in_view_manager(&rect, &mut gamestate.chunks_in_view, &gamestate.player.planet.clone().unwrap().borrow(), &gamestate.itemtypes, &gamestate.item_references);

	hotbar_logic(gamestate);

	
	render::render_planet_chunks(&gamestate.player.planet.clone().unwrap().borrow(),&gamestate.chunks_in_view, &gamestate.itemtypes);

	render_entity(&gamestate.player.planet.clone().unwrap().borrow(), &gamestate.player, &gamestate.texturemanager.imposter);


}

fn render_entity_space(
	entity: &collision::MovableEntity,
	texture: &Texture2D,
){
	draw_texture_ex(
		texture,
		entity.dynrect.rect.center().x,
		entity.dynrect.rect.center().y,
		WHITE,
		DrawTextureParams {
			rotation: entity.rot,
			dest_size: Some(entity.dynrect.rect.size() * 0.1),
			..Default::default()
	}
);
}

fn full_info_planet_to_space_coords(
	planet: &Planet,
	vector: &Vec2,
)-> (Vec2, f32, f32){
	// ITS HERE**2
	let position_x = (((vector.x  *2.0) /(planet.size.x*32) as f32) -1.0) * std::f32::consts::PI;
	let position_y = (vector.y - (planet.size.y*32) as f32) *(std::f32::consts::TAU/(planet.size.x*32) as f32);

	let mut complex_block_position = Complex{re:position_y, im:position_x + *planet.rotation.borrow()};
	
	
	complex_block_position = Complex::exp(complex_block_position);
	 
	let complex: Vec2= Vec2{x: complex_block_position.re, y: complex_block_position.im} * planet.size.x as f32;

	let complex_block_position_x = complex.x + planet.space_position.borrow().x;
	let complex_block_position_y = complex.y + planet.space_position.borrow().y;
	
	let f32complex_block_position = Vec2{x: complex_block_position_x, y: complex_block_position_y};

	let size = f32::sqrt(f32::powf(complex_block_position.re,2.) + f32::powf(complex_block_position.im,2.)) *((std::f32::consts::TAU)/(planet.size.x*32) as f32) * planet.size.x as f32;
	let rotation = complex_block_position.im.atan2(complex_block_position.re) +std::f32::consts::PI/2.;
	(f32complex_block_position, size, rotation)
}

fn inverse_disk_position(vec: Vec2, planet: &Planet) -> Vec2{

	let complex_block_position = Complex {
        re: (vec.y  - planet.space_position.borrow().y) / planet.size.x as f32,
        im: (vec.x  - planet.space_position.borrow().x) / planet.size.x as f32,
    };

    let reversed = complex_block_position.ln();

    let normalisedy = reversed.re;
    let normalisedx: f32 = reversed.im + *planet.rotation.borrow() - std::f32::consts::FRAC_PI_2;
	
    let position_x = ((normalisedx / std::f32::consts::PI) + 1.0) * (planet.size.x * 32)as f32 / 2.0;
    let position_y = (normalisedy / (std::f32::consts::TAU / (planet.size.x * 32)as f32)) + (planet.size.y * 32)as f32;
	Vec2{x: position_x * -1.0, y: position_y}
}



fn rocketmovement(rocket: &mut DynRect, delta: &f32){
	rocket.rect.x += rocket.velocity.x * delta;
	rocket.rect.y += rocket.velocity.y * delta;
}

//its prob gonna give some issues when making the rockets manage who is inside it
fn move_entity_into_spaceship<'a>(
    spaceships: &Vec<Rc<RefCell<SpaceShip<'a>>>>,
    entity: &mut MovableEntity<'a>,
) {
    if !is_key_pressed(KeyCode::Enter) {return;}
    if entity.riding.is_some() {return;}


    for spaceship in spaceships.iter() {
        let spaceship_ref = spaceship.borrow();
        let distance_between = spaceship_ref.entity.dynrect.rect.center().distance(entity.dynrect.rect.center());

        if distance_between > 3.0 {continue;}

        entity.riding = Some(Rc::clone(spaceship));
        break;
    }
}





fn render_spaceships(spaceships: &Vec<Rc<RefCell<SpaceShip>>>, texturemanager: &Texturemanager){


	for spaceship in spaceships.iter(){
		
		let texture:&Texture2D = match spaceship.borrow().which {
			SpaceShipsTypes::Simple => {&texturemanager.simple_spaceship},
			_ => {&texturemanager.imposter},
		};
		
		if spaceship.borrow().entity.planet.is_none(){
			draw_texture(texture, spaceship.borrow().entity.dynrect.rect.center().x, spaceship.borrow().entity.dynrect.rect.center().y, WHITE); 
		continue;};
			render_entity(&spaceship.borrow().entity.planet.clone().unwrap().borrow(), &spaceship.borrow().entity, texture);
	}
}





fn pick_up_items<'a>(player: &collision::MovableEntity<'a>, inventory: &mut [Option<Item>; 10*5], dropped_items: &mut Vec<DroppedItem<'a>>, delta: &f32){
	let mut items_to_remove: Vec<usize> = Vec::new();

	for (iter, dropped_item) in dropped_items.iter().enumerate(){
		if dropped_item.entity.planet != player.planet{continue;}

		let touched = collision::loopingaabb(
			&dropped_item.entity.dynrect.rect, 
			&player.dynrect.rect,
		(player.planet.clone().unwrap().borrow_mut().size.x * 32) as f32, 
			100000.0);
		
		if !(touched){continue;}
		
		items_to_remove.push(iter);

		for (iteriter, bar) in inventory.iter_mut().enumerate(){
			if let Some(existing_item) = bar {
				if existing_item.item_type_id == dropped_item.item.item_type_id{
					existing_item.amount += 1;
					break;
				}
    		}

			if bar.is_some() {continue};
			inventory[iteriter] = Some(dropped_item.item);
			break;
		}
	}
	for &index in items_to_remove.iter().rev() {
        dropped_items.remove(index);
    }
}


//FUCK DOESNT HANDLE ITEMS NOT ON PLANETS. ERRETA FIX TODO FUCK
fn render_dropped_items(dropped_items: &Vec<DroppedItem>, itemtypes: &Vec<ItemType>,){
	

	for dropped_item in dropped_items.iter(){
		if dropped_item.entity.planet.clone().is_none(){continue;}

		if let Some(texture) = itemtypes[0].texture.clone(){
			render_entity(&dropped_item.entity.planet.clone().unwrap().borrow(), &dropped_item.entity, &texture);
		};
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
	None
}



fn hotbar_logic(gamestate: &mut GameState){
	let binding = gamestate.player.planet.clone().unwrap();
 	let planet = &binding.borrow();
	let camera = &gamestate.camera;
	let chunks_in_view = &mut gamestate.chunks_in_view;
	


	let item = gamestate.player_inventory[gamestate.select_hotbar as usize];

	let item = match item {
		Some(x) => x,
		None => return,
	};

	//TODO FIX
	/*
	if is_mouse_button_down(MouseButton::Left) {
		match item {
			Items::DirtBlock { amount: _} => place_block(BlockType::Dirt, camera, planet, chunks_in_view),
			Items::StoneBlock { amount: _ } => place_block(BlockType::Stone, camera, planet, chunks_in_view),
			Items::GrassBlock { amount: _ } => place_block(BlockType::Grass, camera, planet, chunks_in_view),
			Items::PickAxe => destroy_block(camera, gamestate.player.planet.clone().unwrap(), chunks_in_view, &mut gamestate.dropped_items),
			_ => {},
		};
	}*/

}


fn place_block(id_of_block_to_place: usize ,camera: &Camera2D, planet: &Planet, chunks_in_view: &mut HashMap<IVec2,ChunkWithOtherInfo>){
	let camamara = camera.screen_to_world(mouse_position().into());

	let mut cemera:Vec2 = inverse_disk_position(camamara, planet);

	

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
	chunktoread.chunk[blockindex] = id_of_block_to_place;

}

fn destroy_block<'a>(camera: &Camera2D, planet: Rc<RefCell<Planet<'a>>>, chunks_in_view: &mut HashMap<IVec2,ChunkWithOtherInfo>, dropped_items: &mut Vec<DroppedItem<'a>>){
	let camamara = camera.screen_to_world(mouse_position().into());

	let mut cemera:Vec2 = inverse_disk_position(camamara, &planet.borrow());



	if cemera.x <= 0.0 {cemera.x -= 1.0}

	let cemera:IVec2 = IVec2 { x: cemera.x as i32, y: cemera.y as i32 };

	let chunk_x: i32 = cemera.x.rem_euclid(planet.borrow().size.x as i32 * 32).div_euclid(32);
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
	/*
	let item = match chunktoread.chunk[blockindex] {
		BlockType::Stone => Items::StoneBlock { amount: 1 },
		BlockType::Dirt => Items::DirtBlock { amount: 1 },
		BlockType::Grass => Items::GrassBlock { amount: 1 },
		_ => return,
	};
	
	dropped_items.push(DroppedItem {
		entity: MovableEntity {
			dynrect: DynRect {
				rect: Rect {
					x: cemera.x as f32,
					y: cemera.y as f32,
					w: 0.5,
					h: 0.5,
				},
				velocity: Vec2 { x: 0.0, y: 0.0 },
			},
			planet: Some(planet),
			riding: None,
			rot: 0.0,
		},
		items: item,
	});*/
	

	chunktoread.chunk[blockindex] = 0;

}


fn camera_manager_because_fucking_everything_is_broken<'a>(player: &mut collision::MovableEntity<'a>, camera: &mut Camera2D){



	if player.planet.is_some(){
		let transformed = full_info_planet_to_space_coords(&player.planet.clone().unwrap().borrow(), &player.dynrect.rect.center());
		camera.target = transformed.0;
		let zoom: f32 = transformed.1;

		camera.zoom.y = (1.0/screen_height())/zoom;
		camera.zoom.x = (1.0/screen_width())/zoom;

		camera.rotation = -transformed.2 * (360./std::f32::consts::TAU);
		
		if let Some(riding) = &player.riding {
			camera.rotation = -transformed.2 * (360./std::f32::consts::TAU) - riding.borrow().entity.rot * (360./std::f32::consts::TAU);
		}
	}else {
		camera.target.x = player.dynrect.rect.center().x;
		camera.target.y = player.dynrect.rect.center().y;

		let zoom = 0.7;

		camera.zoom.y = (1.0/screen_height())/zoom;
		camera.zoom.x = (1.0/screen_width())/zoom;
		if let Some(riding) = &player.riding {
			camera.rotation = - riding.borrow().entity.rot * (360./std::f32::consts::TAU);
		}
	}




}


fn playermovement(player: &mut DynRect, delta: &f32){
	player.rect.x += player.velocity.x * delta;
	player.rect.y += player.velocity.y * delta;
	player.velocity.x *= 0.98;
	player.velocity.y *= 0.99;
	player.velocity.y -= 19.81* delta;
	if player.velocity.x.abs() < 4.{player.velocity.x *= 0.93;};
}

fn playermovementspace(player: &mut DynRect, delta: &f32){
	player.rect.x += player.velocity.x * delta * 10.0;
	player.rect.y -= player.velocity.y * delta * 10.0;
	player.velocity.x = 0.0;
	player.velocity.y = 0.0;
}


fn render_entity(
	planet: &Planet,
	entity: &collision::MovableEntity,
	texture: &Texture2D,
){ // 99% will need to be more fixed later on
	//Def its own function, should be adapted for any entity

	// This should maybe be put in its own function, could alse be used as a base for render_world() unsure
	//this will be maybe a little more difficoult.
	//world_offset_height must equal a value that makes the entity be in the right y value

	//let player_size = 10.0;
	//println!("{}",playercomplex.re);
	//println!("{:?}", playercomplex);


	//YESYESYESYESYYES
	let transformed = full_info_planet_to_space_coords(planet, &entity.dynrect.rect.center());
	draw_texture_ex(
		texture,
		transformed.0.x - transformed.1 * (entity.dynrect.rect.w/2.0),
		transformed.0.y - transformed.1 * (entity.dynrect.rect.h/2.0),
		WHITE,
		DrawTextureParams {
			dest_size: Some(vec2(transformed.1 * entity.dynrect.rect.w,transformed.1 *entity.dynrect.rect.h)),
			rotation: transformed.2 + entity.rot,
			..Default::default()
	}
);
    
}



fn draw_text_debug(compacta_font:&Font, gamestate:&GameState){
    draw_text_ex(
        format!("{}", get_fps()).as_str(),
        20.0,
        30.0,
        TextParams {
            font_size: 30,
            font: Some(compacta_font),
            ..Default::default()}
        );
		
		draw_text_ex(
			format!("planet center x/y: {}", &gamestate.planets[0].borrow().space_position.borrow()).as_str(),
			20.0,
			60.0,
			TextParams {
				font_size: 30,
				font: Some(compacta_font),
				..Default::default()}
		);
		draw_text_ex(
			format!("player x/y: {}", &gamestate.player.dynrect.rect.point()).as_str(),
			20.0,
			90.0,
			TextParams {
				font_size: 30,
				font: Some(compacta_font),
				..Default::default()}
		);
		draw_text_ex(
			format!("rotation: {}", &gamestate.camera.rotation).as_str(),
			20.0,
			120.0,
			TextParams {
				font_size: 30,
				font: Some(compacta_font),
				..Default::default()}
		);


		let camamara = gamestate.camera.screen_to_world(mouse_position().into());

		let mut cemera:Vec2 = inverse_disk_position(camamara, &gamestate.planets[0].borrow());
	
		if cemera.x <= 0.0 {cemera.x -= 1.0}
	
		let cemera:IVec2 = IVec2 { x: cemera.x as i32, y: cemera.y as i32 };
		draw_text_ex(
			format!("space to planet coords: {}", &cemera).as_str(),
			20.0,
			150.0,
			TextParams {
				font_size: 30,
				font: Some(compacta_font),
				..Default::default()}
		);
}

fn name_plate(compacta_font:&Font, gamestate:&GameState){
	draw_texture_ex(
		&gamestate.texturemanager.nameplate,
		screen_width() - &gamestate.texturemanager.nameplate.width() * 2.0,
		0.0,
		WHITE,
		DrawTextureParams {
			dest_size: Some(vec2(56.0, 20.0) * 2.0),
			..Default::default()
	}
	);
	let text = if gamestate.player.planet.clone().is_some() {gamestate.player.planet.clone().unwrap().borrow().name} else {"space"};
	
    draw_text_ex(
        text,
        screen_width() - &gamestate.texturemanager.nameplate.width() * 2.0 + 8.0,
        32.0,
        TextParams {
            font_size: 30,
            font: Some(compacta_font),
            ..Default::default()}
        );
}

fn rocket_input(rocket: &mut MovableEntity, delta: &f32){


	if is_key_down(KeyCode::A) {
		rocket.rot -= 1.4 * delta;
	}
	if is_key_down(KeyCode::D) {
		rocket.rot += 1.4 * delta;
	}


	if is_key_down(KeyCode::W) {
		rocket.dynrect.velocity.x += rocket.rot.sin() * delta * 5.0;
		rocket.dynrect.velocity.y += rocket.rot.cos() * delta * 5.0;
	}
	if is_key_down(KeyCode::S) {
		rocket.dynrect.velocity.x -= rocket.rot.sin() * delta * 1.0;
		rocket.dynrect.velocity.y -= rocket.rot.cos() * delta * 1.0;
	}
	if rocket.planet.is_none(){
		rocket.dynrect.velocity = rocket.dynrect.velocity.clamp_length_max(500.0 * delta);
	}
}


fn movement_input_space(player: &mut DynRect, delta: &f32){
	if is_key_down(KeyCode::A) {
		player.velocity.x -= 100.0 * delta;
	}
	if is_key_down(KeyCode::D) {
		player.velocity.x += 100.0 * delta;
	}
	if is_key_down(KeyCode::W) {
		player.velocity.y += 100.0 * delta;
	}
	if is_key_down(KeyCode::S) {
		player.velocity.y -= 100.0 * delta;
	}
}


fn render_hotbar(hotebaru: &[Option<Item>;10*5], texturemanager: &Texturemanager, select_hotbar:&i32, itemtypes: &Vec<ItemType>){
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

	for (iter, item) in hotebaru.iter().enumerate().take(10){
		
		let item = match item {
			Some(x) => x,
			None => continue
		};

		let count:Option<u32> = None;

		
		if let Some(texture) = itemtypes[item.item_type_id].texture.clone(){
			draw_texture_ex(&texture,
			dynamic_x_offset - (16.0 * scale) / 2.0 + ((iter as f32) * 18.0 + 10.0) * scale,
			dynamic_y_offset + (16.0 * scale * 0.5) - (6.0 * scale),
			  WHITE,
			   DrawTextureParams{
				   dest_size: Some(vec2(16.0, 16.0) * scale),
				   ..Default::default()}
			);
		};


		

		if let Some(mount) = count{
			draw_text_ex(
			format!("{}", mount).as_str(),
			dynamic_x_offset - (16.0 * scale) / 2.0 + ((iter as f32) * 18.0 + 10.0) * scale,
			dynamic_y_offset + (16.0 * scale * 0.5) - (6.0 * scale) + 16.0 * scale,
			TextParams {
				font_size: 10,
				..Default::default()}
		);
		}
	}
	draw_rectangle_lines(
		dynamic_x_offset - (22.0 * scale) / 2.0 + ((*select_hotbar as f32) * 18.0 + 10.0) * scale,
		dynamic_y_offset + (10.0 * scale * 0.5) - (6.0 * scale),
		22.0 * scale,
		22.0 * scale,
		2.0 * scale,
		BLUE,);
}


fn render_rest_of_the_inventory(inventory: &[Option<Item>;10*5], texturemanager: &Texturemanager, itemtypes: &Vec<ItemType>){
	let scale:f32 = 4.0;

	let dynamic_x_offset = scale*2.0;
	let dynamic_y_offset = scale*2.0;

	for iter in 0..4{
		draw_texture_ex(&texturemanager.hotbar,
			dynamic_x_offset,
			 dynamic_y_offset*3.0 + texturemanager.hotbar.height()*scale + (texturemanager.hotbar.height() * iter as f32) * scale,
			   WHITE,
			    DrawTextureParams{
					dest_size: Some(vec2(texturemanager.hotbar.width(), texturemanager.hotbar.height()) * scale),
					..Default::default()});
	}


	for (iter, item) in inventory.iter().enumerate().skip(10){
		
		let item = match item {
			Some(x) => x,
			None => continue
		};
		let count:Option<u32> = None;

		if let Some(texture) = itemtypes[item.item_type_id].texture.clone(){
			draw_texture_ex(&texture,
			dynamic_x_offset - (16.0 * scale) / 2.0 + (((iter%10) as f32) * 18.0 + 10.0) * scale,
			dynamic_y_offset*3.0 + (16.0 * scale * 0.5) - (6.0 * scale) + (texturemanager.hotbar.height() * (iter/10) as f32) * scale,
			  WHITE,
			   DrawTextureParams{
				   dest_size: Some(vec2(16.0, 16.0) * scale),
				   ..Default::default()}
			);
		};

		

		if let Some(mount) = count{
			draw_text_ex(
			format!("{}", mount).as_str(),
			dynamic_x_offset - (16.0 * scale) / 2.0 + (((iter%10) as f32) * 18.0 + 10.0) * scale,
			dynamic_y_offset*3.0 + (16.0 * scale * 0.5) - (6.0 * scale) + (texturemanager.hotbar.height() * (iter/10) as f32) * scale + 16.0 * scale,
			TextParams {
				font_size: 10,
				..Default::default()}
		);
		}
	}
}