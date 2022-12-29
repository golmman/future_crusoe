//use controller::Controller;
//
//pub mod controller;
//pub mod model;
//pub mod view;
//
//fn main() {
//    let controller = Controller::new();
//    term2d::run(controller);
//}

use rand::thread_rng;
use rand::Rng;

const WIDTH: usize = 80;
const HEIGHT: usize = 50;

struct Cave {
    grid: [[bool; WIDTH]; HEIGHT],
}

impl Cave {
    fn new() -> Cave {
        // Create a new grid of cells, randomly setting each cell to on or off
        let mut grid = [[false; WIDTH]; HEIGHT];
        let mut rng = thread_rng();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                grid[y][x] = rng.gen();
            }
        }

        Cave { grid }
    }

    fn step(&mut self) {
        // Create a new grid to store the updated state of the cells
        let mut new_grid = [[false; WIDTH]; HEIGHT];

        // Loop through each cell and apply the rules to determine its new state
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let neighbors = self.count_neighbors(x, y);
                new_grid[y][x] = self.apply_rules(self.grid[y][x], neighbors);
            }
        }

        // Update the grid with the new state of the cells
        self.grid = new_grid;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        // Count the number of on cells in a 3x3 grid centered around the given cell
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let x = (x as isize + dx) as usize;
                let y = (y as isize + dy) as usize;
                if x < WIDTH && y < HEIGHT && self.grid[y][x] {
                    count += 1;
                }
            }
        }
        count
    }

    fn apply_rules(&self, cell: bool, neighbors: u8) -> bool {
        // Turn off a cell if it has less than 4 neighbors, or turn on a cell if it has more than 5 neighbors
        if cell && (neighbors < 4 || neighbors > 5) {
            false
        } else if !cell && neighbors > 4 {
            true
        } else {
            cell
        }
    }
}

fn main() {
    let mut cave = Cave::new();

    // Run the cellular automaton for a number of steps
    for _ in 0..3 {
        cave.step();
    }

    // Print the resulting cave to the console
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if cave.grid[y][x] {
                print!("{}", "#");
            } else {
                print!("{}", " ");
            }
        }
        println!();
    }
}
