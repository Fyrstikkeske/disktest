use std::{cell::RefCell, collections::HashMap};

use macroquad::math::{Rect, Vec2, IVec2,};

use crate::chunk::{BlockType, ChunkWithOtherInfo, Planet};

pub struct MovableEntity<'a>{
    pub dynrect: DynRect,
    pub planet: Option<std::rc::Rc<RefCell<Planet<'a>>>>,
    pub riding: Option<std::rc::Rc<RefCell<crate::SpaceShip<'a>>>>,
}

struct Ray{
    origin:Vec2,
    direction:Vec2
}

pub struct DynRect{
	pub rect:Rect,
    pub velocity:Vec2,
}

struct RayRectInfo{
    hit: bool,
    contact_point:Vec2,
    contact_normal:Vec2,
    t_hit_near:f32,
}

fn ray_vs_rect(
    ray:&Ray,
    rect: &Rect,
        ) -> RayRectInfo{
    let mut ray_rect_info = RayRectInfo{
        hit: false,
        contact_point: Vec2{x:0.0,y: 0.0}, 
        contact_normal: Vec2{x:0.0,y: 0.0}, 
        t_hit_near: 0.0};

    let mut t_near = (rect.point() - ray.origin) / ray.direction;
    let mut t_far = (rect.point() + rect.size() - ray.origin) / ray.direction;
    
    if t_near.x > t_far.x { std::mem::swap( &mut t_near.x, &mut t_far.x)};
    if t_near.y > t_far.y { std::mem::swap( &mut t_near.y, &mut t_far.y)};
    
    if t_far.y.is_nan() || t_far.x.is_nan() {return ray_rect_info};
    if t_near.y.is_nan() || t_near.x.is_nan() {return ray_rect_info};

    if t_near.x > t_far.y || t_near.y > t_far.x {return ray_rect_info};

    ray_rect_info.t_hit_near = f32::max(t_near.x, t_near.y);
    let t_hit_far = f32::min(t_far.x, t_far.y);

    if t_hit_far <0.0 {return ray_rect_info};

    ray_rect_info.contact_point = ray.origin + ray_rect_info.t_hit_near * ray.direction;

    if t_near.x > t_near.y{
        if ray.direction.x < 0.0{
            ray_rect_info.contact_normal = Vec2 {x: 1.0,y: 0.0}
        }else{
            ray_rect_info.contact_normal = Vec2 {x: -1.0,y: 0.0}
        }
    }else if t_near.x < t_near.y {
        if ray.direction.y < 0.0{
            ray_rect_info.contact_normal = Vec2 {x: 0.0,y: 1.0}
        }else{
            ray_rect_info.contact_normal = Vec2 {x: 0.0,y: -1.0}
        }
    }
    ray_rect_info.hit = true;
    ray_rect_info
}



fn dynamic_rect_vs_rect(
    rect:&Rect,
    dynrect: &DynRect,
    delta: &f32,
        ) -> RayRectInfo
{
    let mut ray_rect_info = RayRectInfo{hit: false, contact_point: Vec2{x:0.0,y: 0.0}, contact_normal: Vec2{x:0.0,y: 0.0}, t_hit_near: 0.0};

    if dynrect.velocity.x == 0.0 && dynrect.velocity.y == 0.0{
        return ray_rect_info;
    }

    
    let exp_rect_pos = rect.point() - dynrect.rect.size() / 2.;
    let exp_rect_size = rect.size() + dynrect.rect.size();
    let expanded_target:Rect = Rect { x: exp_rect_pos.x, y: exp_rect_pos.y, w: exp_rect_size.x, h: exp_rect_size.y };

    ray_rect_info = ray_vs_rect(
        &Ray{ origin: dynrect.rect.point() + dynrect.rect.size()/2.0, direction: dynrect.velocity * *delta},
        &expanded_target
    );

    if ray_rect_info.hit{
        if ray_rect_info.t_hit_near <= 1.0 && ray_rect_info.t_hit_near >= 0.0{ 
            ray_rect_info.hit = true;
            return ray_rect_info;
    }}

    ray_rect_info.hit = false;
    ray_rect_info
}


//fix(done), chatgpt fixed this
pub fn dynamic_rectangle_vs_planet_chunks(
    delta: &f32,
    dynrect: &mut DynRect,
    chunks_in_view: &HashMap<IVec2,ChunkWithOtherInfo>,
    planet: &crate::chunk::Planet,
) {
    let future_dynrect_position_x: f32 = dynrect.rect.x + (dynrect.velocity.x * *delta);
    let future_dynrect_position_y: f32 = dynrect.rect.y + (dynrect.velocity.y * *delta);

    let combined_block = dynrect.rect.combine_with(Rect {
        x: future_dynrect_position_x,
        y: future_dynrect_position_y,
        w: dynrect.rect.w,
        h: dynrect.rect.h,
    });

    let search_rectangle = Rect {
        x: combined_block.x.floor(),
        y: combined_block.y.floor(),
        w: combined_block.right().ceil() - combined_block.x.floor(),
        h: combined_block.bottom().ceil() - combined_block.y.floor(),
    };

    let area: usize = (search_rectangle.w * search_rectangle.h) as usize;
    let mut collisions_with: Vec<(usize, f32, Rect)> = vec![];

    for i in 0..area {
        let x = (i % search_rectangle.w as usize) as i32 + search_rectangle.x as i32;
        let y = (i / search_rectangle.w as usize) as i32 + search_rectangle.y as i32;

        let chunk_x: i32 = x.div_euclid(32);
        let chunk_y: i32 = y.div_euclid(32);
        if chunk_y >= planet.size.y as i32{continue}
        let chunktoread = chunks_in_view.get(&IVec2 { x: chunk_x.rem_euclid(planet.size.x as i32), y: chunk_y.rem_euclid(planet.size.y as i32) });

        let chunktoread = match chunktoread {
            Some(chunk) => chunk,
            None => {
                eprintln!(
                    "Trying to access a chunk that doesn't exist for collision at {} {}",
                    chunk_x, chunk_y
                );
                continue;
            }
        };

        let blockindex: usize = (x.rem_euclid(32) + (y.rem_euclid(32)) * 32) as usize;
        
        match chunktoread.chunk[blockindex] {
            BlockType::Dirt | BlockType::Grass | BlockType::Stone => {}
            _ => continue,
        }

        let block = Rect {
            x: x as f32,
            y: y as f32,
            w: 1.0,
            h: 1.0,
        };
        let ray_rect_info = dynamic_rect_vs_rect(&block, dynrect, delta);

        if ray_rect_info.hit {
            collisions_with.push((blockindex, ray_rect_info.t_hit_near, block));
        }
    }

    collisions_with.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for round in collisions_with {
        let x = round.2.x;
        let y = round.2.y;

        let element = Rect {
            x: x as f32,
            y: y as f32,
            w: 1.0,
            h: 1.0,
        };
        let ray_rect_info = dynamic_rect_vs_rect(&element, dynrect, &delta);
        if ray_rect_info.hit {
            dynrect.velocity += ray_rect_info.contact_normal * dynrect.velocity.abs()
                * (1.0 - ray_rect_info.t_hit_near);
        }
    }
}
