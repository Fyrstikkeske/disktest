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