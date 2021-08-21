use sdl2::event::Event;
use sdl2::keyboard::Keycode;
pub mod lib;
use crate::lib::types::{Ant, Grid};

fn main() {
    let (mut canvas, mut events) = lib::init(700, 700); // Create a window
    let mut ant = Ant::new((35, 35), 3); // Create an instance of an ant
    let (rows, columns) = (70, 70); // The number of rows and columns on the board
    let mut grid = Grid::new(rows, columns); // The board
    let cell_width = 10; // The size in pixels of each cell

    'game: loop {
        // If escape is pressed, the window closes
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                _ => continue 'game,
            }
        }
        // get current ant position
        let (y, x) = ant.position;

        // if the ant leaves the board stop the game
        if (y >= rows || x >= columns) || (y == 0 || x == 0) {
            continue 'game;
        } else {
            ant.move_ant(grid.grid[y as usize][x as usize]);

            if grid.grid[y as usize][x as usize] == 1 {
                grid.grid[y as usize][x as usize] = 0;
            } else {
                grid.grid[y as usize][x as usize] = 1;
            }
        }

        // show the board in the window
        lib::display_frame(&mut canvas, &grid, &ant, &columns, &rows, &cell_width);
    }
}
