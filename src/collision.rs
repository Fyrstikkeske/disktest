use std::{cell::RefCell, collections::HashMap, mem::swap};

use macroquad::math::{vec2, IVec2, Rect, Vec2};

use crate::chunk::{BlockType, ChunkWithOtherInfo, Planet};
#[derive(Debug)]
pub struct MovableEntity<'a>{
    pub dynrect: DynRect,
    pub planet: Option<std::rc::Rc<RefCell<Planet<'a>>>>,
    pub riding: Option<std::rc::Rc<RefCell<crate::SpaceShip<'a>>>>,
    pub rot: f32,
}

struct Ray{
    origin:Vec2,
    direction:Vec2
}
#[derive(Debug)]
pub struct DynRect{
	pub rect:Rect,
    pub velocity:Vec2,
}

#[derive(Default,)]
pub struct RayRectInfo{
    pub hit: bool,
    contact_point:Vec2,
    contact_normal:Vec2,
    t_hit_near:f32,
}

fn ray_vs_rect(ray: &Ray, rect: &Rect) -> RayRectInfo {
    let mut t_near = (rect.point() - ray.origin) / ray.direction;
    let mut t_far = (rect.point() + rect.size() - ray.origin) / ray.direction;

    if t_near.x > t_far.x { swap(&mut t_near.x, &mut t_far.x); }
    if t_near.y > t_far.y { swap(&mut t_near.y, &mut t_far.y); }

    if t_far.x.is_nan() || t_far.y.is_nan() || t_near.x.is_nan() || t_near.y.is_nan() { return RayRectInfo::default(); }
    if t_near.x > t_far.y || t_near.y > t_far.x { return RayRectInfo::default(); }

    let t_hit_near = t_near.x.max(t_near.y);
    if t_hit_near < 0.0 || t_far.x.min(t_far.y) < 0.0 { return RayRectInfo::default(); }

    let contact_normal = if t_near.x > t_near.y {
        if ray.direction.x.is_sign_negative() { vec2(1.0, 0.0) } else { vec2(-1.0, 0.0) }
    } else {
        if ray.direction.y.is_sign_negative() { vec2(0.0, 1.0) } else { vec2(0.0, -1.0) }
    };

    RayRectInfo {
        hit: true,
        contact_point: ray.origin + t_hit_near * ray.direction,
        contact_normal,
        t_hit_near,
    }
}

pub fn looping_dynamic_rect_vs_rect(rect: &Rect, dynrect: &DynRect, delta: f32, width: f32, height: f32) -> RayRectInfo {
    let x_offsets = [0.0, width, -width];
    let y_offsets = [0.0, height, -height];
    
    let mut earliest_hit = RayRectInfo::default();

    for &x_offset in &x_offsets {
        for &y_offset in &y_offsets {
            let shifted_rect = Rect {
                x: rect.x + x_offset,
                y: rect.y + y_offset,
                w: rect.w,
                h: rect.h,
            };

            let info = dynamic_rect_vs_rect(&shifted_rect, dynrect, delta);
            if info.hit && (earliest_hit.hit == false || info.t_hit_near < earliest_hit.t_hit_near) {
                earliest_hit = info;
                break;
            }
        }
    }
    earliest_hit
}

pub(crate) 
fn dynamic_rect_vs_rect(rect: &Rect, dynrect: &DynRect, delta: f32) -> RayRectInfo {
    if dynrect.velocity == Vec2::ZERO {
        return RayRectInfo::default();
    }


    let expanded_rect = Rect::new(
        rect.x - dynrect.rect.w / 2.0,
        rect.y - dynrect.rect.h / 2.0,
        rect.w + dynrect.rect.w,
        rect.h + dynrect.rect.h,
    );

    let ray = Ray {
        origin: dynrect.rect.point() + dynrect.rect.size() / 2.0,
        direction: dynrect.velocity * delta,
    };
    
    let mut rayrectinfo = ray_vs_rect(&ray, &expanded_rect);
    if rayrectinfo.hit && (rayrectinfo.t_hit_near >= 0.0 && rayrectinfo.t_hit_near <= 1.0) {
        return rayrectinfo; 
    }

    rayrectinfo.hit = false;
    rayrectinfo
}


//fix(done), chatgpt fixed this
pub fn dynamic_rectangle_vs_planet_chunks(
    delta: &f32,
    dynrect: &mut DynRect,
    chunks_in_view: &HashMap<IVec2,ChunkWithOtherInfo>,
    planet: &crate::chunk::Planet,
) -> bool{
    let mut collisiondetected = false;
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
                continue;/*
                eprintln!(
                    "Trying to access a chunk that doesn't exist for collision at {} {}",
                    chunk_x, chunk_y
                );*/
                
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
        let ray_rect_info = dynamic_rect_vs_rect(&block, dynrect, *delta);

        if ray_rect_info.hit {
            collisions_with.push((blockindex, ray_rect_info.t_hit_near, block));
        }
    }

    collisions_with.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for round in collisions_with {
        let x = round.2.x;
        let y = round.2.y;

        let element = Rect {
            x: x,
            y: y,
            w: 1.0,
            h: 1.0,
        };
        let ray_rect_info = dynamic_rect_vs_rect(&element, dynrect, *delta);
        if ray_rect_info.hit {
            dynrect.velocity += ray_rect_info.contact_normal * dynrect.velocity.abs()
                * (1.0 - ray_rect_info.t_hit_near);
                collisiondetected = true;
        }
    }
    collisiondetected
}
