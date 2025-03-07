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

	let planet_me = Planet{
		name: "PlanetMe",
		space_position: RefCell::new(Vec2{x: 000.0, y: 10000.0}),
		size: UVec2 { x: 10, y: 20},
		rotation: RefCell::new(0.0),
	};
	

	let mut stellar_objects: Vec<Planet> = Vec::new();

	
	stellar_objects.push(planet_me);
	
	let mut player:collision::MovableEntity = collision::MovableEntity{
		dynrect: collision::DynRect{rect:Rect{x:1.0, y: 620.0, w: 1.75, h:2.6}, velocity: Vec2::ZERO},
		planet: None,
		riding: None,
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
		},
		items:Items::PickAxe,
	};

	let spaceship:Rc<RefCell<SpaceShip>> =  Rc::new(RefCell::new(SpaceShip{
		entity: collision::MovableEntity{
			dynrect: collision::DynRect{rect:Rect{x:-10.0, y: 620.0, w: 5.75, h:10.3}, velocity: Vec2::ZERO},
			planet: None,
			riding: None,
		},
	fuel: 100,
	which: SpaceShipsTypes::Simple,
	}));

	let mut chunks_in_view:HashMap<IVec2,ChunkWithOtherInfo> = HashMap::new();
	let mut dropped_items:Vec<DroppedItem> = vec![drop];
	let mut space_ships: Vec<Rc<RefCell<SpaceShip>>> = vec![spaceship];

	let mut player_hotbar:[Option<Items>;10] = [None; 10];
	let mut select_hotbar:i32 = 1;


	player_hotbar[0] = Some(Items::PickAxe);
	player_hotbar[1] = Some(Items::DirtBlock { amount: 1 });
	player_hotbar[9] = Some(Items::DirtBlock { amount: 1 });

	let mut gamestate: GameState = GameState{
		planets: Vec::new(),
		player: player,
		camera: Camera2D::default(),
		delta: 0.0,
		chunks_in_view: HashMap::new(),
		texturemanager: texturemanager::texture_manager().await,
	};

	gamestate.planets.push(terra);
	gamestate.player.planet = Some(gamestate.planets[0].clone());

	//THATS WHY HE IS THE GOAT!!! THE GOAT!!!!!!
    loop{
		gamestate.delta = get_frame_time();
    	clear_background(BLACK);

		//*gamestate.player.planet.unwrap().rotation.borrow_mut() += 0.001;
		
		if is_key_pressed(KeyCode::Enter) {
			gamestate.player.dynrect.rect.move_to(vector_from_planet_to_space_coords(&gamestate.player.planet.clone().unwrap().borrow(), &gamestate.player.dynrect.rect.center()) - gamestate.player.dynrect.rect.size()/2.0);
			gamestate.player.planet = None;
			//gamestate.camera.rotation += 90.0;
		}
		
		if gamestate.player.planet.is_some(){
			on_planet(&mut gamestate);
		}

		if gamestate.player.planet.is_none(){
			off_planet(&mut gamestate)
		}


		

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
		match keyboard_number() {
			Some(number) => select_hotbar = number as i32,
			None =>{}
		};

		
		

		/* 
    	camera_zoom *= zoom;
		camera_target = player.dynrect.rect.center();
    	let mut camera = Camera2D {
        	zoom: camera_zoom,
        	target: camera_target,
			rotation: camera_rotation,
        	..Default::default()
    	};*/

		

		/*if player.planet.is_some(){

		}else {
			space_camera(player.riding.clone().unwrap().borrow().entity.dynrect.rect.center(),  &mut camera.target, &mut camera_zoom, &mut camera_rotation);
		}*/
		


		

		/*if player.planet.is_some(){
			//hotbar_logic(&camera, &player.planet.unwrap(), &mut chunks_in_view, &player_hotbar, &select_hotbar);

			//make it so that i only render the world the player is on, The situation in where he can see 2 planets at the same time should never happen
			//something like this render_world(player.planet), shit also need to add a point in which to see
			
		}*/



		render_spaceships(&space_ships, &texturemanager);
		render_dropped_items(&dropped_items, &texturemanager);

		/*if player.planet.is_some(){
			
		} else {
			draw_texture(&texturemanager.imposter, player.dynrect.rect.x, player.dynrect.rect.y, WHITE)
		}*/


		set_default_camera();


		render_hotbar(&player_hotbar, &texturemanager, &select_hotbar);
    	draw_text_debug(&compacta_font, &gamestate);
		//gamestate.camera = Camera2D::default();
    	next_frame().await
    }
}
fn off_planet(gamestate:&mut GameState){
	*gamestate.planets[0].borrow_mut().rotation.borrow_mut() += 0.001;
	gamestate.planets[0].borrow_mut().space_position.borrow_mut().x -= 0.001;
	let rect:Rect = Rect{
		x: 20.0,
    	y: 630.0,
    	w: 20.0,
    	h: 1.0,
	};
	chunk::chunks_in_view_manager(&rect, &mut gamestate.chunks_in_view, &gamestate.planets[0].borrow());

	space_camera(gamestate.player.dynrect.rect.center(), &mut gamestate.camera.target, &mut gamestate.camera.zoom, &mut gamestate.camera.rotation);
	set_camera(&gamestate.camera);


	
	render::render_planet_chunks(&gamestate.planets[0].borrow(),&gamestate.chunks_in_view, &gamestate.texturemanager);

	render_entity_space(&gamestate.player, &gamestate.texturemanager.imposter);
}

fn on_planet(gamestate:&mut GameState){
	*gamestate.player.planet.clone().unwrap().borrow_mut().rotation.borrow_mut() += 0.001;
	gamestate.planets[0].borrow_mut().space_position.borrow_mut().x -= 0.001;
	if gamestate.player.riding.is_none() {
		movement_input(&mut gamestate.player.dynrect, &gamestate.delta);
	}
	collision::dynamic_rectangle_vs_planet_chunks(
		&gamestate.delta, 
		&mut gamestate.player.dynrect,
		&gamestate.chunks_in_view,
		&gamestate.player.planet.clone().unwrap().borrow());
	playermovement(&mut gamestate.player.dynrect, &gamestate.delta);
	
	
	let rect:Rect = Rect{
		x: gamestate.player.dynrect.rect.center().x,
    	y: gamestate.player.dynrect.rect.center().y,
    	w: 4.0,
    	h: 2.0,
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
			dest_size: Some(entity.dynrect.rect.size()*0.01),
			..Default::default()
	}
);
}

fn vector_from_planet_to_space_coords(
	planet: &Planet,
	vector: &Vec2,
)-> Vec2{
	// ITS HERE
	let normalisedentityx = ((((vector.x - 0.5) *2.0) /(planet.size.x*32) as f32) -1.0) * std::f32::consts::PI;
	let normalisedentityy = ((vector.y - 0.5) - (planet.size.y*32) as f32) *(std::f32::consts::TAU/(planet.size.x*32) as f32);

	let mut playercomplex = Complex{re:normalisedentityy, im:normalisedentityx + *planet.rotation.borrow()};
	

	playercomplex = Complex::exp(playercomplex);
	let player_node_x = playercomplex.re + planet.space_position.borrow().x;
	let player_node_y = playercomplex.im + planet.space_position.borrow().y;
	Vec2{x: player_node_x, y: player_node_y}
}


fn space_camera(position: Vec2, camera_pos: &mut Vec2, camera_zoom: &mut Vec2, camera_rotation: &mut f32){

	camera_pos.x = position.x;
	camera_pos.y = position.y;

	let zoom = 0.001;

	camera_zoom.y = (1.0/screen_height())/zoom;
	camera_zoom.x = (1.0/screen_width())/zoom;

	
	//*camera_rotation = (camera_pos.x.atan2(camera_pos.y) * (360./std::f32::consts::TAU)) + 180.;
}



fn rocketmovement(rocket: &mut DynRect, delta: &f32){
	rocket.rect.x = rocket.rect.x + (rocket.velocity.x * delta);
	rocket.rect.y = rocket.rect.y + (rocket.velocity.y * delta);
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
			Items::DirtBlock { amount } => {&texturemanager.dirt},
			Items::GrassBlock { amount } => {&texturemanager.grass},
			Items::PickAxe => {&texturemanager.pickaxe},
			Items::StoneBlock { amount } => {&texturemanager.stone},
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

	let mut playercomplex = Complex{re:normalisedplayery, im:normalisedplayerx + *planet.rotation.borrow()};

	
	playercomplex = Complex::exp(playercomplex);

	camera_pos.x = playercomplex.re + planet.space_position.borrow().x;
	camera_pos.y = playercomplex.im + planet.space_position.borrow().y;

	let zoom = f32::sqrt(f32::powf(playercomplex.re,2.)+f32::powf(playercomplex.im,2.)) *(std::f32::consts::TAU/(planet.size.x*32) as f32);

	camera_zoom.y = (1.0/screen_height())/zoom;
	camera_zoom.x = (1.0/screen_width())/zoom;

	
	*camera_rotation = (playercomplex.re.atan2(playercomplex.im) * (360./std::f32::consts::TAU)) + 180.;
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

	let mut playercomplex = Complex{re:normalisedplayery, im:normalisedplayerx + *planet.rotation.borrow()};
	

	playercomplex = Complex::exp(playercomplex);
	let player_node_x = playercomplex.re + planet.space_position.borrow().x;
	let player_node_y = playercomplex.im + planet.space_position.borrow().y;
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
			rotation: playercomplex.im.atan2(playercomplex.re)+std::f32::consts::FRAC_PI_2,
			..Default::default()
	}
);
    
}

fn inverse_disk_position(vec: Vec2, planet: &Planet) -> Vec2{

	let complex = Complex{re: vec.y, im: vec.x};

    let reversed = complex.ln();

    let normalisedy = reversed.re;
    let normalisedx = reversed.im + *planet.rotation.borrow() - std::f32::consts::FRAC_PI_2;
	
    let position_x = ((normalisedx / std::f32::consts::PI) + 1.0) * (planet.size.x * 32)as f32 / 2.0;
    let position_y = (normalisedy / (std::f32::consts::TAU / (planet.size.x * 32)as f32)) + (planet.size.y * 32)as f32;
	Vec2{x: position_x * -1.0, y: position_y}
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
fn rocket_input(rocket: &mut DynRect, delta: &f32, zoom: &mut f32){
	if is_key_down(KeyCode::A) {
		rocket.velocity.x -= 100.0 * delta;
	}
	if is_key_down(KeyCode::D) {
		rocket.velocity.x += 100.0 * delta;
	}
	if is_key_down(KeyCode::W) {
		rocket.velocity.y += 100.0 * delta;
	}
	if is_key_down(KeyCode::S) {
		rocket.velocity.y -= 100.0 * delta;
	}
	if is_key_down(KeyCode::KpAdd) {
		*zoom -= 4.0 * delta;
	}
	if is_key_down(KeyCode::KpSubtract) {
		*zoom += 4.0 * delta;
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