use sdl2::event::Event;
use sdl2::keyboard::Keycode;
pub mod lib;
use crate::lib::types::{Ant, Grid};

fn main() {
    let (mut canvas, mut events) = lib::init(700, 700); // Crear la ventana
    let mut ant = Ant::new((35, 35), 3); // Crear una nueva hormiga
    let (rows, columns) = (70, 70); // La cantidad de filas y columnas en el tablero
    let mut grid = Grid::new(rows, columns); // El tablero
    let cell_width = 10; // El tamaño de cada selda

    'game: loop {
        // Si se aprieta escape, se cierra la ventana
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
        let (y, x) = ant.position;

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

        lib::display_frame(&mut canvas, &grid, &ant, &columns, &rows, &cell_width);
    }
}
