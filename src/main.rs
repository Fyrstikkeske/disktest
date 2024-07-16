use macroquad::prelude::*;
use num_complex::{Complex, ComplexFloat};


#[derive(Clone, Copy, PartialEq)]
enum BlockType {
	Air,
	Stone,
	Dirt,
	Grass,
	Marvin,
}

struct Ray{
    origin:Vec2,
    direction:Vec2
}

struct DynRect{
	rect:Rect,
    velocity:Vec2,
}

struct RayRectInfo{
    hit: bool,
    contact_point:Vec2,
    contact_normal:Vec2,
    t_hit_near:f32,
}

struct World<'a>{
	x_size: usize,
	y_size: usize,
	blocks: &'a mut [BlockType],
}

struct Texturemanager{
	dirt: Texture2D,
	imposter: Texture2D,
	stone: Texture2D,
	grass: Texture2D,
}

const ARRX:usize = 30;
const ARRY:usize = 30;


#[macroquad::main("Torus")]
async fn main() {
	let mut player = DynRect{rect:Rect{x:1.0, y: 30.0, w: 1.0, h:1.0}, velocity: Vec2::ZERO};

	let mut world_offset_rotation:f32 = 0.0;
	let mut world_offset_height:f32 = 6.0;
	let mut world_offset_global_x:f32 = 960.0;
	let mut world_offset_global_y:f32 = 540.0;

    println!("Hello, universe!");

    let mut hyperboria:World = World{x_size: ARRX, y_size: ARRY, blocks: &mut [BlockType::Stone; ARRX*ARRY]};

	let moon:World = World{x_size: 25, y_size: 25, blocks: &mut [BlockType::Stone; 25*25]};

	for i in 0..hyperboria.x_size{
		hyperboria.blocks[(hyperboria.x_size * hyperboria.y_size)-(i+1)] = BlockType::Grass;
	}

	
	let texturemanager = Texturemanager{
		dirt: load_texture("textures/dirt.png").await.unwrap(),
		imposter: load_texture("textures/imposter.png").await.unwrap(),
		stone: load_texture("textures/stone.png").await.unwrap(),
		grass: load_texture("textures/grass.png").await.unwrap(),
	};

	texturemanager.stone.set_filter(FilterMode::Nearest);
	texturemanager.dirt.set_filter(FilterMode::Nearest);
	texturemanager.grass.set_filter(FilterMode::Nearest);


    loop{
    	clear_background(BLACK);
		let delta = get_frame_time();


		if is_key_down(KeyCode::Right) {
            world_offset_rotation -= std::f32::consts::TAU/hyperboria.y_size as f32;
        }

		if is_key_down(KeyCode::Left) {
            world_offset_rotation += std::f32::consts::TAU/hyperboria.y_size as f32;
        }

		if is_key_down(KeyCode::Down) {
            world_offset_height += std::f32::consts::TAU/hyperboria.x_size as f32;
        }

		if is_key_down(KeyCode::Up) {
            world_offset_height -= std::f32::consts::TAU/hyperboria.x_size as f32;
        }

		if is_key_down(KeyCode::A) {
            player.velocity.x -= 40.0 * delta;
        }

		if is_key_down(KeyCode::D) {
            player.velocity.x += 40.0 * delta;
        }

		if is_key_down(KeyCode::W) {
			world_offset_global_y -= 100.;
        }

		if is_key_down(KeyCode::S) {
			world_offset_global_y += 100.;
        }

		if is_key_pressed(KeyCode::Space) {
			player.velocity.y += 10.00;
        }

		
		dynamic_rectangle_vs_world(&delta, &mut player, &mut hyperboria);

		let normalisedplayerx = (player.rect.x *2.0 /hyperboria.x_size as f32 -1.0) * std::f32::consts::PI;
		let normalisedplayery = (player.rect.y - hyperboria.y_size as f32) *((std::f32::consts::PI*2.)/hyperboria.x_size as f32);

		let mut playercomplex = Complex{re:normalisedplayery + world_offset_height, im:normalisedplayerx + world_offset_rotation};
		playercomplex = Complex::exp(playercomplex);
		let player_node_x = playercomplex.re;
		let player_node_y = playercomplex.im;
		let player_size = f32::sqrt(f32::powf(playercomplex.re,2.)+f32::powf(playercomplex.im,2.)) *((std::f32::consts::PI*2.)/ARRX as f32);


		render_world(&hyperboria, &texturemanager, world_offset_height, world_offset_rotation, world_offset_global_x, world_offset_global_y);

		render_world(&moon, &texturemanager, 4.0, 0.0, 100.0, 200.0);

		player.rect.x = player.rect.x + (player.velocity.x * delta);
    	player.rect.y = player.rect.y + (player.velocity.y * delta);

		player.velocity.x = player.velocity.x * 0.96;
		player.velocity.y -= 9.81* delta;

		if player.velocity.x.abs() < 4.{player.velocity.x = player.velocity.x * 0.89;};

		draw_texture_ex(
				&texturemanager.imposter,
				player_node_x - player_size/2. + world_offset_global_x,
				player_node_y - player_size/2. + world_offset_global_y,
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(player_size,player_size*player.rect.h)),
					rotation: player_node_y.atan2(player_node_x)+std::f32::consts::PI/2.,
					..Default::default()
				}
			);
    	next_frame().await
    }
}

fn dynamic_rectangle_vs_world(delta:&f32,dynrect:&mut DynRect, world:&mut World){

	dynrect.rect.x = dynrect.rect.x.rem_euclid(world.x_size as f32);

	let future_dynrect_position_x:f32 = dynrect.rect.x + (dynrect.velocity.x* *delta);
	let future_dynrect_position_y:f32 = dynrect.rect.y + (dynrect.velocity.y* *delta);


	let combined_block = dynrect.rect.combine_with(Rect{
		x: future_dynrect_position_x, 
		y: future_dynrect_position_y,
		w: dynrect.rect.w,
		h: dynrect.rect.h,
	});

	

	let mut search_rectangle = Rect{
		x: combined_block.x.floor(),
		y: combined_block.y.floor(),
		w: combined_block.right().ceil() - combined_block.x.floor(),
		h: combined_block.bottom().ceil() - combined_block.y.floor(),
	};

	let area:usize =(search_rectangle.w * search_rectangle.h) as usize; 

	let mut collisions_with:Vec<(usize,f32)> = vec![];

	for i in 0..area{
		let x = (i%search_rectangle.w as usize) + search_rectangle.x.rem_euclid(world.x_size as f32) as usize;
		let y = (i/search_rectangle.w as usize) + search_rectangle.y as usize;
		if y >= world.y_size{
			continue;
		}
		

		let blockindex = (x%world.x_size)+y*world.x_size;

		match world.blocks[blockindex] {
			BlockType::Dirt =>{}
			BlockType::Grass =>{}
			BlockType::Stone =>{}
			_ =>{continue;}
		}
	

		let block = Rect{x: (x%world.x_size) as f32, y: y as f32, w: 1.0, h: 1.0};
        let rayrectinfo = dynamic_rect_vs_rect(&block, dynrect, delta);
		
        if rayrectinfo.hit{
            collisions_with.push((blockindex,rayrectinfo.t_hit_near));
        }
        
		
		
		//world.blocks[blockindex] = BlockType::Dirt;
	}

	collisions_with.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

	for round in collisions_with{
		let x = round.0%world.x_size;
		let y = round.0/world.x_size;
		if y >= world.y_size{
			continue;
		}
		

		let element = Rect{x: x as f32, y: y as f32, w: 1.0, h: 1.0};
		let rayrectinfo = dynamic_rect_vs_rect(&element, dynrect, &delta);
		if rayrectinfo.hit{
			dynrect.velocity += rayrectinfo.contact_normal * dynrect.velocity.abs() * (1.0-rayrectinfo.t_hit_near);
		}
	}

}



fn render_world
(
	world : &World, 
	texturemanager: &Texturemanager,
	world_offset_height: f32, 
	world_offset_rotation: f32,
	world_offset_global_x: f32,
	world_offset_global_y: f32,
){	

	for i in 0..(world.x_size*world.y_size){
		let x = i%world.x_size;
		let y = i/world.x_size;


			//should be obvius
			match world.blocks[i] {
				BlockType::Air => {continue}
				BlockType::Marvin => {continue}
				_ => {}
			}

			if world.blocks[i] == BlockType::Air{
				continue;
			}

			let normalised_block_position = Vec2{
				x:(x as f32 *2.0 /world.x_size as f32 -1.0) * std::f32::consts::PI,
				y: ((y) as f32 - world.y_size as f32) *((std::f32::consts::PI*2.)/world.x_size as f32)
			};



			let pre_complex_block_position = Complex{re:normalised_block_position.y + world_offset_height, im:normalised_block_position.x + world_offset_rotation};


			let complex_block_position = Complex::exp(pre_complex_block_position);
			let block_x = complex_block_position.re;
			let block_y = complex_block_position.im;
			let size = f32::sqrt(f32::powf(block_x,2.)+f32::powf(block_y,2.)) *((std::f32::consts::PI*2.)/world.x_size as f32);

			let texture_to_use:&Texture2D;
			let rotation = block_y.atan2(block_x) +std::f32::consts::PI/2.;


			match world.blocks[i] {
				BlockType::Dirt => {texture_to_use = &texturemanager.dirt}
				BlockType::Grass => {texture_to_use = &texturemanager.grass}
				BlockType::Stone => {texture_to_use = &texturemanager.stone}
				_ => {continue;}
			}

			draw_texture_ex(
				texture_to_use,
				block_x - size/2. + world_offset_global_x,
				block_y - size/2. + world_offset_global_y,
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(size,size)),
					rotation: rotation,
					..Default::default()
				},
			);	

		
	}
}

fn dynamic_rect_vs_rect(
    rect:&Rect,
    dynrect: &DynRect,
    delta: &f32,
        ) -> RayRectInfo
{
    let mut RayRectInfo = RayRectInfo{hit: false, contact_point: Vec2{x:0.0,y: 0.0}, contact_normal: Vec2{x:0.0,y: 0.0}, t_hit_near: 0.0};

    if dynrect.velocity.x == 0.0 && dynrect.velocity.y == 0.0{
        return RayRectInfo;
    }

    
    let exp_rect_pos = rect.point() - dynrect.rect.size() / 2.;
    let exp_rect_size = rect.size() + dynrect.rect.size();
    let expanded_target:Rect = Rect { x: exp_rect_pos.x, y: exp_rect_pos.y, w: exp_rect_size.x, h: exp_rect_size.y };

    RayRectInfo = ray_vs_rect(
        &Ray{ origin: dynrect.rect.point() + dynrect.rect.size()/2.0, direction: dynrect.velocity * *delta},
        &expanded_target
    );

    if RayRectInfo.hit{
        if RayRectInfo.t_hit_near <= 1.0 && RayRectInfo.t_hit_near >= 0.0{ 
            RayRectInfo.hit = true;
            return RayRectInfo;
    }}

    RayRectInfo.hit = false;
    RayRectInfo
}

fn ray_vs_rect(
    ray:&Ray,
    rect: &Rect,
        ) -> RayRectInfo{
    let mut RayRectInfo = RayRectInfo{
        hit: false,
        contact_point: Vec2{x:0.0,y: 0.0}, 
        contact_normal: Vec2{x:0.0,y: 0.0}, 
        t_hit_near: 0.0};

    let mut t_near = (rect.point() - ray.origin) / ray.direction;
    let mut t_far = (rect.point() + rect.size() - ray.origin) / ray.direction;
    
    if t_near.x > t_far.x { std::mem::swap( &mut t_near.x, &mut t_far.x)};
    if t_near.y > t_far.y { std::mem::swap( &mut t_near.y, &mut t_far.y)};
    
    if t_far.y.is_nan() || t_far.x.is_nan() {return RayRectInfo};
    if t_near.y.is_nan() || t_near.x.is_nan() {return RayRectInfo};

    if t_near.x > t_far.y || t_near.y > t_far.x {return RayRectInfo};

    RayRectInfo.t_hit_near = f32::max(t_near.x, t_near.y);
    let t_hit_far = f32::min(t_far.x, t_far.y);

    if t_hit_far <0.0 {return RayRectInfo};

    RayRectInfo.contact_point = ray.origin + RayRectInfo.t_hit_near * ray.direction;

    if t_near.x > t_near.y{
        if ray.direction.x < 0.0{
            RayRectInfo.contact_normal = Vec2 {x: 1.0,y: 0.0}
        }else{
            RayRectInfo.contact_normal = Vec2 {x: -1.0,y: 0.0}
        }
    }else if t_near.x < t_near.y {
        if ray.direction.y < 0.0{
            RayRectInfo.contact_normal = Vec2 {x: 0.0,y: 1.0}
        }else{
            RayRectInfo.contact_normal = Vec2 {x: 0.0,y: -1.0}
        }
    }
    RayRectInfo.hit = true;
    RayRectInfo
}