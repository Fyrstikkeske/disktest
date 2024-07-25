use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
	Air,
	Stone,
	Dirt,
	Grass,
	Marvin,
}


pub struct World<'a>{
	pub x_size: usize,
	pub y_size: usize,
	pub blocks: &'a mut [BlockType],
}

pub struct Planet<'a>{
    pub space_position: &'a mut Vec2,
    pub name: &'a str,
    pub size: IVec2,
	pub rotation: f32,
}


struct Chunk{
    position: IVec2,
    chunk: [BlockType; 1024],
}
