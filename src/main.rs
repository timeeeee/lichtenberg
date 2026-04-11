use array2d::Array2D;
use rand::prelude::*;

/*
const WALK_STEP_LENGTH: f32 = 1.0;

const WORLD_SIZE: i32 = 256;
const BOX_SIZE: i32 = 4;

struct Point {
    x: f32,
    y: f32
}
 */

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

struct Point {
    x: usize,
    y: usize,
    parent: (usize, usize)
}


fn get_neighbor(particle_x: usize, particle_y: usize, cells: &Array2D<bool>) -> Option<(usize, usize)> {
    for (dx, dy) in DIRECTIONS {
	let Some(x) = particle_x.checked_add_signed(dx as isize) else { continue };
	let Some(y) = particle_y.checked_add_signed(dy as isize) else { continue };

	if cells.get(x, y) == Some(&true) { return Some((x, y)) };
    }
    
    None
}


fn pixel_walk(width: usize, height: usize) -> Vec<Point> {
    /* moving pixel by pixel */

    let mut rng = rand::rng();
    
    let mut tree = Vec::<Point>::new();

    // start with one pixel basically center that is stationary
    let mut is_stationary = Array2D::filled_by_row_major(|| true, width, height);
    is_stationary[(width / 2, height / 2)] = true;

    for _ in 0..10000 {
	// just pick a random pixel just anywhere and reject it if it's already next to somebody?
	let x = rng.random_range(0..(width as i32));
	let y = rng.random_range(0..(height as i32));

	if is_stationary[(x, y)] { continue };
	if get_neighbor(x as usize, y as usize, &is_stationary).is_some() { continue };

	// now walk
	loop {
	    let (dx, dy) = DIRECTIONS.choose(&mut rng).unwrap();
	    x += dx;
	    y += dy;

	    // if we left the window, start over with a new point
	    if (x < 0) | (x >= width) | (y < 0) | (y >= height) { continue };

	    // if we have a new neighbor, stop here
	    match get_neighbor(x as usize, y as usize, &is_stationary) {
		Some((neighbor_x, neighbor_y)) => {
		    is_stationary[(x, y)] = true;
		    tree.push(Point {
			x: x,
			y: y,
			parent: (neighbor_x, neighbor_y)
		    })},
		_ => {}
	    }

	    // otherwise keep on walkin'
	}
    };

    tree
}

fn main() {
    // for speeding up collision checks
    // let mut boxes = Array2D::filled_by_row_major(Vec::<Point>::new, 64, 64);

    println!("hiiii");
}
