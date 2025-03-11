use core::f32;
use std::{cell::RefCell, collections::HashMap, rc::Rc};


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

// need this or else i will do something evil.
struct GameState<'a>{
	planets: Vec<Rc<RefCell<Planet<'a>>>>,
	player: collision::MovableEntity<'a>,
	camera: Camera2D, 
	delta: f32,
	chunks_in_view: HashMap<IVec2,ChunkWithOtherInfo>,
	texturemanager: texturemanager::Texturemanager,
	player_hotbar:[Option<Items>;10],
	select_hotbar: i32,
	spaceships: Vec<Rc<RefCell<SpaceShip<'a>>>>,
}

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
#[derive(Debug)]
enum SpaceShipsTypes{
	Sigma,
	Ohio,
	Simple,
	IUseALotOfSpaceships,
}

struct DroppedItem<'a>{
	entity: collision::MovableEntity<'a>,
	items: Items,
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
		rotation: RefCell::new(0.0),
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

    let mut zoom:f32 = 48.0;
    let mut camera_rotation:f32 = 0.0;
    let mut camera_zoom = Vec2{x:1./10.0, y:1./10.0};
    let mut camera_target:Vec2 = Vec2 { x: 0.0, y: 0.0 };

	let drop:DroppedItem = DroppedItem{
		entity: collision::MovableEntity{
			dynrect: collision::DynRect{rect:Rect{x:1.4, y: 625.0, w: 1.7, h:1.7}, velocity: Vec2::ZERO},
			planet: None,
			riding: None,
			rot: 0.0 
		},
		items:Items::PickAxe,
	};

	let sigma_spaceship:Rc<RefCell<SpaceShip>> =  Rc::new(RefCell::new(SpaceShip{
		entity: collision::MovableEntity{
			dynrect: collision::DynRect{rect:Rect{x:-10.0, y: 620.0, w: 5.75, h:10.0}, velocity: Vec2::ZERO},
			planet: None,
			riding: None,
			rot: 0.0,
		},
	fuel: 100,
	which: SpaceShipsTypes::Simple,
	}));

	let mut chunks_in_view:HashMap<IVec2,ChunkWithOtherInfo> = HashMap::new();
	let mut dropped_items:Vec<DroppedItem> = vec![drop];
	//let mut space_ships: Vec<Rc<RefCell<SpaceShip>>> = vec![spaceship];

	let mut player_hotbar:[Option<Items>;10] = [None; 10];
	


	player_hotbar[0] = Some(Items::PickAxe);
	player_hotbar[1] = Some(Items::DirtBlock { amount: 1 });
	player_hotbar[2] = Some(Items::StoneBlock { amount: 1 });
	player_hotbar[3] = Some(Items::GrassBlock { amount: 1 });
	player_hotbar[9] = Some(Items::DirtBlock { amount: 1 });

	let mut gamestate: GameState = GameState{
		planets: Vec::new(),
		player: player,
		camera: Camera2D::default(),
		delta: 0.0,
		chunks_in_view: HashMap::new(),
		texturemanager: texturemanager::texture_manager().await,
		player_hotbar: player_hotbar,
		select_hotbar: 1,
		spaceships: Vec::new(),
	};

	gamestate.planets.push(terra);
	gamestate.planets.push(planet_me);
	gamestate.player.planet = Some(gamestate.planets[0].clone());
	gamestate.spaceships.push(sigma_spaceship);
	gamestate.spaceships[0].borrow_mut().entity.planet = Some(gamestate.planets[0].clone());

	//THATS WHY HE IS THE GOAT!!! THE GOAT!!!!!!
    loop{
		gamestate.delta = get_frame_time();
    	clear_background(BLACK);

		
		planets_system(&mut gamestate);
		
		if gamestate.player.planet.is_some(){
			on_planet(&mut gamestate);
		}else {
			off_planet(&mut gamestate)
		}
		

		spaceship_system(&mut gamestate);

		

		/* 
		if player.planet.is_none() && player.riding.is_some(){
			rocket_input_space(&mut player.riding.clone().unwrap().borrow_mut().entity.dynrect, &delta, &mut zoom);
		};
		
		if player.riding.is_some() {
			rocketmovement(&mut player.riding.clone().unwrap().borrow_mut().entity.dynrect, &delta);
		}*/

		/*if player.planet.is_none(){
		for stellar_object in stellar_objects.iter(){
			let distance = player.riding.clone().unwrap().borrow_mut().entity.dynrect.rect.point().distance(*stellar_object.space_position.borrow());
			if distance+1000.0 > (stellar_object.size.x * 32) as f32{continue;}

			player.planet = Some(stellar_object);
			player.riding.clone().unwrap().borrow_mut().entity.planet = Some(stellar_object);
		};}; */
		

		
		

		//pick_up_items(&gamestate.player, &mut player_hotbar, &mut dropped_items);
		//move_entity_into_spaceship(&space_ships, &mut gamestate.player);
		if let Some(number) = keyboard_number() {
			gamestate.select_hotbar = number as i32 
		}

	



		//render_spaceships(&space_ships, &texturemanager);
		render_dropped_items(&dropped_items, &texturemanager);


		set_default_camera();


		render_hotbar(&player_hotbar, &texturemanager, &gamestate.select_hotbar);
		if true {
			draw_text_debug(&compacta_font, &gamestate);
		}
    	
		name_plate(&compacta_font, &gamestate);
		//gamestate.camera = Camera2D::default();
    	next_frame().await
    }
}
fn spaceship_system(gamestate:&mut GameState){
	if gamestate.player.planet.is_some() && is_key_pressed(KeyCode::Enter){
		if gamestate.player.riding.is_some() {
			gamestate.player.riding = None;
		}else {
			for spaceship in &gamestate.spaceships{
				if spaceship.borrow().entity.planet.is_none(){ continue;}
				let distance = gamestate.player.dynrect.rect.center().distance(spaceship.borrow().entity.dynrect.rect.center());
				if distance > 5.0{continue;}
				gamestate.player.riding = Some(spaceship.clone());
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
				spaceship.entity.rot += *planet.borrow().rotation.borrow() - f32::consts::FRAC_PI_2;
				return;
			}

			collision::dynamic_rectangle_vs_planet_chunks(
				&gamestate.delta,
				&mut spaceship.entity.dynrect,
				&gamestate.chunks_in_view,
				&planet.borrow());
			spaceship.entity.dynrect.rect.x += spaceship.entity.dynrect.velocity.x * gamestate.delta;
			spaceship.entity.dynrect.rect.y += spaceship.entity.dynrect.velocity.y * gamestate.delta;
			spaceship.entity.dynrect.rect.x = spaceship.entity.dynrect.rect.x.rem_euclid((spaceship.entity.planet.clone().unwrap().borrow().size.x * 32) as f32);
		}else {
			for planet in gamestate.planets.clone(){
				let mut cemera:Vec2 = inverse_disk_position(spaceship.entity.dynrect.rect.center(), &planet.borrow()) + 0.5;

				if cemera.x <= 0.0 {cemera.x -= 1.0}
			
				if cemera.y as u32 <= &planet.borrow().size.y * 32{
					spaceship.entity.planet = Some(planet.clone());
					spaceship.entity.dynrect.rect.move_to(cemera);
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
		*planet.borrow_mut().rotation.borrow_mut() += 0.01;

        let planet = planet.borrow_mut();
        let mut rotation_mut = planet.rotation.borrow_mut();
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

	space_camera(gamestate.player.dynrect.rect.center(), &mut gamestate.camera.target, &mut gamestate.camera.zoom, &mut gamestate.camera.rotation);
	set_camera(&gamestate.camera);




	for planet in gamestate.planets.clone(){
		let mut cemera:Vec2 = inverse_disk_position(gamestate.player.dynrect.rect.center(), &planet.borrow()) + 0.5;

		if cemera.x <= 0.0 {cemera.x -= 1.0}
	
		if cemera.y as u32 <= &planet.borrow().size.y * 32{
			gamestate.player.planet = Some(planet.clone());
			gamestate.player.dynrect.rect.move_to(cemera);
			return;
		}
		
		//gamestate.planets[0].borrow_mut().space_position.borrow_mut().x -= 0.001;

		let rect:Rect = Rect{
			x: cemera.x,
			y: cemera.y,
			w: 4.0,
			h: 6.0,
		};

		chunk::chunks_in_view_manager(&rect, &mut gamestate.chunks_in_view, &planet.borrow());
		render::render_planet_chunks(&planet.borrow(),&gamestate.chunks_in_view, &gamestate.texturemanager);
	}

	if gamestate.player.riding.is_none(){
		render_entity_space(&gamestate.player, &gamestate.texturemanager.imposter);
	}
}

fn on_planet(gamestate:&mut GameState){
	//*gamestate.player.planet.clone().unwrap().borrow_mut().rotation.borrow_mut() += 0.001;
	//gamestate.planets[0].borrow_mut().space_position.borrow_mut().x -= 0.001;
	
	if gamestate.player.dynrect.rect.y > (gamestate.player.planet.clone().unwrap().borrow().size.y * 32 + 16) as f32 {
		chunk::save_planet_from_manager(&mut gamestate.chunks_in_view, &gamestate.player.planet.clone().unwrap().borrow());
		gamestate.player.dynrect.rect.move_to(full_info_planet_to_space_coords(&gamestate.player.planet.clone().unwrap().borrow(), &gamestate.player.dynrect.rect.center()).0 - gamestate.player.dynrect.rect.size()/2.0);
		gamestate.player.planet = None;
		gamestate.player.dynrect.velocity = Vec2::ZERO;
		gamestate.camera.rotation = 0.0;
		return;
	}
	
	gamestate.player.dynrect.rect.x = gamestate.player.dynrect.rect.x.rem_euclid((gamestate.player.planet.clone().unwrap().borrow().size.x * 32) as f32);

	if let Some(spaceship) = &gamestate.player.riding{
		gamestate.player.dynrect.rect.move_to(spaceship.borrow().entity.dynrect.rect.center() - gamestate.player.dynrect.rect.size()/2.0);
	}else {
		movement_input(&mut gamestate.player.dynrect, &gamestate.delta);
		collision::dynamic_rectangle_vs_planet_chunks(
			&gamestate.delta,
			&mut gamestate.player.dynrect,
			&gamestate.chunks_in_view,
			&gamestate.player.planet.clone().unwrap().borrow());
		playermovement(&mut gamestate.player.dynrect, &gamestate.delta);
	}


	
	

	let rect:Rect = Rect{
		x: gamestate.player.dynrect.rect.center().x,
    	y: gamestate.player.dynrect.rect.center().y,
    	w: 4.0,
    	h: 6.0,
	};
	chunk::chunks_in_view_manager(&rect, &mut gamestate.chunks_in_view, &gamestate.player.planet.clone().unwrap().borrow());
	

	
	set_camera_target_to_position_planet(
		gamestate.player.dynrect.rect.center(), 
		&gamestate.player.planet.clone().unwrap().borrow(), 
		&mut gamestate.camera.target, 
		&mut gamestate.camera.zoom, 
		&mut gamestate.camera.rotation);
	
	gamestate.camera.zoom *= 48.;
	set_camera(&gamestate.camera);

	hotbar_logic(&gamestate.camera, &gamestate.player.planet.clone().unwrap().borrow(), &mut gamestate.chunks_in_view, &gamestate.player_hotbar, &gamestate.select_hotbar);

	
	render::render_planet_chunks(&gamestate.player.planet.clone().unwrap().borrow(),&gamestate.chunks_in_view, &gamestate.texturemanager);

	render_entity(&gamestate.player.planet.clone().unwrap().borrow(), &gamestate.player, &gamestate.texturemanager.imposter);
	
	
	/*else {
		set_camera_target_to_position_planet(player.riding.clone().unwrap().borrow().entity.dynrect.rect.center(), &player.planet.unwrap(), &mut camera.target, &mut camera_zoom, &mut camera_rotation);
	}*/

	/* else {
		rocket_input(&mut player.riding.clone().unwrap().borrow_mut().entity.dynrect, &delta, &mut zoom);
		
		if player.riding.clone().unwrap().borrow().entity.dynrect.rect.bottom() > player.riding.clone().unwrap().borrow().entity.planet.unwrap().size.y as f32 * 32.0 + 32.0{
			

			let rot = ((player.dynrect.rect.x - 0.5)  *2.0 /(player.planet.unwrap().size.x*32) as f32 -1.0) * std::f32::consts::PI;

			player.riding.clone().unwrap().borrow_mut().entity.dynrect.rect.x = f32::sin(rot) * (player.planet.unwrap().size.x*32) as f32;
			player.riding.clone().unwrap().borrow_mut().entity.dynrect.rect.y = f32::sin(rot) * (player.planet.unwrap().size.y*32) as f32;

			player.dynrect.rect.x = player.riding.clone().unwrap().borrow_mut().entity.dynrect.rect.x;
			player.dynrect.rect.y = player.riding.clone().unwrap().borrow_mut().entity.dynrect.rect.y;

			player.riding.clone().unwrap().borrow_mut().entity.planet = None;
			player.planet = None;

		}
	}
	*/


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
			dest_size: Some(entity.dynrect.rect.size()*0.1),
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
    let normalisedx = reversed.im + *planet.rotation.borrow() - std::f32::consts::FRAC_PI_2;
	
    let position_x = ((normalisedx / std::f32::consts::PI) + 1.0) * (planet.size.x * 32)as f32 / 2.0;
    let position_y = (normalisedy / (std::f32::consts::TAU / (planet.size.x * 32)as f32)) + (planet.size.y * 32)as f32;
	Vec2{x: position_x * -1.0, y: position_y}
}

fn space_camera(position: Vec2, camera_pos: &mut Vec2, camera_zoom: &mut Vec2, _camera_rotation: &mut f32){

	camera_pos.x = position.x;
	camera_pos.y = position.y;

	let zoom = 0.01;

	camera_zoom.y = (1.0/screen_height())/zoom;
	camera_zoom.x = (1.0/screen_width())/zoom;

	
	//*camera_rotation = (camera_pos.x.atan2(camera_pos.y) * (360./std::f32::consts::TAU)) + 180.;
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
			draw_texture(texture, spaceship.borrow().entity.dynrect.rect.center().x, spaceship.borrow().entity.dynrect.rect.center().y, WHITE); continue;};

		render_entity(&spaceship.borrow().entity.planet.clone().unwrap().borrow(), &spaceship.borrow().entity, texture);
	}
}





fn pick_up_items<'a>(player: &collision::MovableEntity<'a>, hotebaru: &mut [Option<Items>; 10], dropped_items: &mut Vec<DroppedItem<'a>>){
	let mut items_to_remove: Vec<usize> = Vec::new();

	for (iter, dropped_item) in dropped_items.iter().enumerate(){
		if dropped_item.entity.planet != player.planet{continue;}

		let distance_between = player.dynrect.rect.center().distance(dropped_item.entity.dynrect.rect.center());
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
		if dropped_item.entity.planet.clone().is_none(){continue;}
		let texture:&Texture2D = match dropped_item.items {
			Items::DirtBlock { amount: _ } => {&texturemanager.dirt},
			Items::GrassBlock { amount: _ } => {&texturemanager.grass},
			Items::PickAxe => {&texturemanager.pickaxe},
			Items::StoneBlock { amount: _ } => {&texturemanager.stone},
			_ => {&texturemanager.imposter},
		};
		render_entity(&dropped_item.entity.planet.clone().unwrap().borrow(), &dropped_item.entity, texture);
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



fn hotbar_logic(camera: &Camera2D, planet: &Planet, chunks_in_view: &mut HashMap<IVec2,ChunkWithOtherInfo>, hotebaru: &[Option<Items>;10], select_hotbar:&i32){
	let item = hotebaru[*select_hotbar as usize];

	let item = match item {
		Some(x) => x,
		None => return,
	};



	if is_mouse_button_down(MouseButton::Left) {
		match item {
			Items::DirtBlock { amount: _} => place_block(BlockType::Dirt, camera, planet, chunks_in_view),
			Items::StoneBlock { amount: _ } => place_block(BlockType::Stone, camera, planet, chunks_in_view),
			Items::GrassBlock { amount: _ } => place_block(BlockType::Grass, camera, planet, chunks_in_view),
			Items::PickAxe => destroy_block(camera, planet, chunks_in_view),
			
		};
	}

}


fn place_block(block_type: BlockType,camera: &Camera2D, planet: &Planet, chunks_in_view: &mut HashMap<IVec2,ChunkWithOtherInfo>){
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
	chunktoread.chunk[blockindex] = block_type;

}

fn destroy_block(camera: &Camera2D, planet: &Planet, chunks_in_view: &mut HashMap<IVec2,ChunkWithOtherInfo>){
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

	let transformed = full_info_planet_to_space_coords(planet, &position);
	*camera_pos = transformed.0;
	let zoom = transformed.1;

	camera_zoom.y = (1.0/screen_height())/zoom;
	camera_zoom.x = (1.0/screen_width())/zoom;

	
	*camera_rotation = -transformed.2 * (360./std::f32::consts::TAU);
}


fn playermovement(player: &mut DynRect, delta: &f32){
	player.rect.x += player.velocity.x * delta;
	player.rect.y += player.velocity.y * delta;
	player.velocity.x *= 0.96;
	player.velocity.y *= 0.96;
	player.velocity.y -= 9.81* delta;
	if player.velocity.x.abs() < 4.{player.velocity.x *= 0.89;};
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

		let mut cemera:Vec2 = inverse_disk_position(camamara, &gamestate.planets[0].borrow()) + 0.5;
	
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
fn rocket_input_space(rocket: &mut DynRect, delta: &f32, zoom: &mut f32){
	if is_key_down(KeyCode::A) {
		rocket.velocity.x -= 5.0 * delta;
	}
	if is_key_down(KeyCode::D) {
		rocket.velocity.x += 5.0 * delta;
	}
	if is_key_down(KeyCode::W) {
		rocket.velocity.y += 5.0 * delta;
	}
	if is_key_down(KeyCode::S) {
		rocket.velocity.y -= 5.0 * delta;
	}
}
fn rocket_input(rocket: &mut MovableEntity, delta: &f32){


	if is_key_down(KeyCode::A) {
		rocket.rot -= 1.4 * delta;
	}
	if is_key_down(KeyCode::D) {
		rocket.rot += 1.4 * delta;
	}


	if is_key_down(KeyCode::W) {
		rocket.dynrect.velocity.x += rocket.rot.sin() * delta * 10.0;
		rocket.dynrect.velocity.y += rocket.rot.cos() * delta * 10.0;
	}
	if is_key_down(KeyCode::S) {
		rocket.dynrect.velocity.x -= rocket.rot.sin() * delta * 10.0;
		rocket.dynrect.velocity.y -= rocket.rot.cos() * delta * 10.0;
	}
	if rocket.planet.is_none(){
		rocket.dynrect.velocity = rocket.dynrect.velocity.clamp_length_max(500.0 * delta);
	}
}
fn movement_input(player: &mut DynRect, delta: &f32){
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
			Items::DirtBlock { amount: _ } => &texturemanager.dirt,
			Items::StoneBlock { amount: _ } => &texturemanager.stone,
			Items::GrassBlock { amount: _ } => &texturemanager.grass,
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