use macroquad::prelude::*;
use std::{collections::HashMap, fs, io::{self}};
use rand::srand;
use std::cell::RefCell;

pub const CHUNKSIZE:usize = 1024;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlockType {
	Air,
	Stone,
	Dirt,
	Grass,
	Marvin,
}

#[derive(PartialEq, Debug)]
pub struct Planet<'a>{
    pub space_position: RefCell<Vec2>,
	pub rotation: RefCell<f32>,
    pub name: &'a str,
    pub size: UVec2,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChunkWithOtherInfo{
    pub position: IVec2,
    pub chunk: [BlockType; CHUNKSIZE],
}


pub fn readchunkfile(position: IVec2, planet: &Planet) -> ChunkWithOtherInfo{

    let mut chunk:[BlockType; CHUNKSIZE] = [BlockType::Air; CHUNKSIZE];

    let file_to_read_from = "Planets".to_string() + "/" + planet.name  + "/x" + &position.x.to_string() + "y" + &position.y.to_string();

    let chunkstring = match fs::read_to_string(file_to_read_from){
        Ok(chunkstring) => chunkstring,
        Err(err) => {
            match err.kind() {
                io::ErrorKind::NotFound => {
                    //just check if the planets folder exist. might not which is the worst case
                    //eprintln!("{} most likely not generated chunks at {} yet", err, position); 
                    return ChunkWithOtherInfo{position: position, chunk: generate_chunk(6, position, planet)};}
                _ => {
                    eprintln!("{} at chunk {}", err, position); 
                    return ChunkWithOtherInfo{position: position, chunk: [BlockType::Air; 1024]};}
            }
        }
    };
    
    let tokens: Vec<&str> = chunkstring.split("+").collect();

    assert_eq!(tokens.len(), 1024);

    for (iteration, &item) in tokens.iter().enumerate(){
        let blocksenum = match item {
            "Air" => BlockType::Air,
            "Dirt" => BlockType::Dirt,
            "Stone" => BlockType::Stone,
            "Grass" => BlockType::Grass,
            _ => {eprintln!("could not read the block. Chunk:{} Iter:{}",position , iteration); BlockType::Air},
        };

        chunk[iteration] = blocksenum;
    }

    ChunkWithOtherInfo{position: position, chunk: chunk}
}

fn generate_chunk(seed:i32, position: IVec2, planet: &Planet) -> [BlockType; CHUNKSIZE] {
    let mut chunk = [BlockType::Air; CHUNKSIZE];
    srand((seed as i128 + position.x as i128 + position.y as i128) as u64);

    let randomnumber = rand::gen_range(0, 100);

    for (iter, blocktype) in chunk.iter_mut().enumerate(){
        let local_x:i32 = iter as i32%32;
        let local_y:i32 = iter as i32/32;
        let planet_x:i32 = local_x + position.x*32;
        let planet_y:i32 = local_y + position.y*32;

        let sinex:f32 = f32::sin((planet_x as f32/5.0) + randomnumber as f32/100.);

        if planet_y == 0{
            *blocktype = BlockType::Stone;
            continue;
        }

        if planet_y == 100{
            *blocktype = BlockType::Air;
            continue;
        }

        if planet_y == 99{
            *blocktype = BlockType::Air;
            continue;
        }
        if planet_x == (planet.size.x * 32) as i32 -33 {
            *blocktype = BlockType::Stone;
            continue;
        }
        if planet_x == (planet.size.x * 32) as i32 -32 {
            *blocktype = BlockType::Stone;
            continue;
        }
        if planet_x == (planet.size.x * 32) as i32 -31 {
            *blocktype = BlockType::Air;
            continue;
        }
        if planet_x == 10{
            *blocktype = BlockType::Air;
            continue;
        }
        if planet_x == 11{
            *blocktype = BlockType::Air;
            continue;
        }

        if planet_y == (planet.size.y*32) as i32 -33{
            *blocktype = BlockType::Grass;
            continue;
        }
        if planet_y < (planet.size.y*32) as i32 - 33 && planet_y as f32 > ((sinex+1.)*8.0) + 140.0 {
            *blocktype = BlockType::Stone;
            continue;
        }

    }
    
    chunk
}

pub fn writechunkfile(chunk_info: ChunkWithOtherInfo, planet: &Planet){
    let mut chunkstring = "".to_string();

    for i in 0..CHUNKSIZE{
        let blockstring = match chunk_info.chunk[i] {
            BlockType::Air => "Air",
            BlockType::Dirt => "Dirt",
            BlockType::Stone => "Stone",
            BlockType::Grass => "Grass",
			BlockType::Marvin => "Marvin",
        };

        if i != 1023{
            chunkstring.push_str(&(blockstring.to_owned() + "+"));
            continue;
        }  
        chunkstring.push_str(blockstring)
    }

    let file_to_write_to = "Planets/".to_string() + planet.name  + "/x" + &chunk_info.position.x.to_string() + "y" + &chunk_info.position.y.to_string();
    
    
    match fs::write(&file_to_write_to, chunkstring){
        Ok(_) => {}
        Err(err) => {
            
            let planets_folder_to_save_to = "Planets/".to_string() + planet.name;
            let location_to_planet_folder = fs::read_dir(&planets_folder_to_save_to);
            match location_to_planet_folder {
                Ok(_) => eprintln!("Error unable to save file, planets folder exist though. so idk maybe out of space to save chunk?: {}. ERR:{}", file_to_write_to, err),
                Err(_) => {
                    match fs::create_dir(planets_folder_to_save_to){
                        Ok(_) => {},
                        Err(err) => eprintln!("planets folder doesnt exist, and OS refuse to make it: {}. ERR:{}", file_to_write_to, err),
                    }
                },
            }
        },
    }
}

pub fn chunks_in_view_manager(display: &Rect, chunks_in_view: &mut HashMap<IVec2,ChunkWithOtherInfo>, planet:&Planet){
    /*let planet = match planet {
		Some(theplanet) => theplanet,
		None => {eprintln!("WHY TF ARE YOU TRYING TO CHUNK SOMETHING THATS NOT A PLANET"); return;}
	}; */

    

    let x:f32 = ((display.x - 32.) / 32.) + (1.5 - display.w/2.0);
    let y:f32 = ((display.y - 32.) / 32.) + (1.5 - display.h/2.0);

    let search_rectangle = Rect{
		x: x.floor() ,
		y: y.floor() ,
		w: display.w,
		h: display.h,
	};

    //println!("{}",display.y);

    //draw_rectangle(search_rectangle.x, search_rectangle.y, search_rectangle.w, search_rectangle.h, RED);

    let area:usize =(search_rectangle.w * search_rectangle.h).ceil() as usize;

    let mut chunktoremove = chunks_in_view.clone();

    for i in 0..area{
		let x:i32 = ((i as i32 %search_rectangle.w as i32) + search_rectangle.x as i32).rem_euclid(planet.size.x as i32);
		let y:i32 = (i as i32 /search_rectangle.w as i32 + search_rectangle.y as i32).rem_euclid(planet.size.y as i32);


        chunktoremove.remove(&IVec2{x: x, y: y});


        match chunks_in_view.get(&IVec2{x: x, y: y}){
            Some(_)=> {}
            None => {
                if y > planet.size.y as i32 {chunks_in_view.insert(IVec2{x: x, y: y},ChunkWithOtherInfo{chunk: [BlockType::Air; CHUNKSIZE], position: IVec2 { x: (i as i32 %search_rectangle.w as i32) + search_rectangle.x as i32, y: i as i32 /search_rectangle.w as i32 + search_rectangle.y as i32 }});}
                else{chunks_in_view.insert(IVec2{x: x, y: y}, ChunkWithOtherInfo{chunk: readchunkfile(IVec2{x: x, y: y}, planet).chunk, position: IVec2 { x: (i as i32 %search_rectangle.w as i32) + search_rectangle.x as i32, y: i as i32 /search_rectangle.w as i32 + search_rectangle.y as i32 }});}
            }
        }
    }
    for (key, chunk) in chunktoremove{
        writechunkfile(ChunkWithOtherInfo{position: key, chunk:chunk.chunk}, planet);
        chunks_in_view.remove(&key);
    }
}

pub fn save_planet_from_manager(chunks_in_view: &mut HashMap<IVec2,ChunkWithOtherInfo>, planet:&Planet){
    
    for (key, chunk) in chunks_in_view{
        writechunkfile(ChunkWithOtherInfo{position: *key, chunk:chunk.chunk}, planet);
    }
}
