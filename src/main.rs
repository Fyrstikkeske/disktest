use macroquad::prelude::*;
use num_complex::Complex;


const ARRX:usize = 10;
const ARRY:usize = 10;

#[macroquad::main("Torus")]
async fn main() {
    println!("Hello, world!");

    let mut array:[[Vec2;ARRY];ARRX] = [[Vec2{x:0.0,y: 0.0};ARRY];ARRX];
    let r_p = 0.1585 * array.len() as f32;
    
    for x in 0..ARRX{
    	for y in 0..ARRY{
    		array[x][y] = Vec2{x:x as f32 * 1.,y: y as f32 * 1.};
    	}
    }




    
    loop{
    	clear_background(BLACK);
    	for x in 0..ARRX{
    		for y in 0..ARRY{
    			let mut complex = Complex{re:array[x][y].y + array.len() as f32/2., im:array[x][y].x};
    			complex = r_p * Complex::exp(complex/r_p);
				let node_x = complex.re;
				let node_y = complex.im;
				
				draw_circle( node_x/50. + 500.0,  node_y/50. + 500.0, f32::sqrt(f32::powf(node_x,2.)+f32::powf(node_y,2.))/400., Color{r:0.1 * x as f32, g:0.1* y as f32, b:1.0, a:1.0});
    		}
    	}

    	next_frame().await
    }
}


