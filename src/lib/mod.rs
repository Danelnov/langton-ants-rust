use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
pub mod types;
use types::{Grid, Ant};

/// Create a new window
pub fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Ant", width + 1, height + 1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}

/// Draw a cell at a specific position
pub fn display_cell(
    renderer: &mut Canvas<Window>,
    row: u32,
    col: u32,
    grid_data: &Grid,
    cell_width: &u32,
    ant_data: &Ant,
) {
    let cell_height = cell_width;

    let grid = &grid_data.grid;

    let x = cell_width * col;
    let y = cell_width * row;

    // if the cell is equal to the ant's position, the cell will be red
    // if the cell has a value of 1, it will be painted white
    // if it has a value of 0 it will be painted black
    let drawing_color = if (row, col) == ant_data.position {
        Color::RGB(255, 0, 0)
    } else {
        let cell = grid[row as usize][col as usize];
        if cell == 1 {
            Color::RGB(255, 255, 255)
        } else {
            Color::RGB(0, 0, 0)
        }
    };

    // draw the picture
    renderer.set_draw_color(drawing_color);
    let square = renderer.fill_rect(Rect::new(x as i32, y as i32, *cell_width, *cell_height));
    match square {
        Ok(()) => {}
        Err(error) => println!("{}", error),
    }
}

/// Draw all cells in the window
pub fn display_frame(
    renderer: &mut Canvas<Window>,
    grid: &Grid,
    ant: &Ant,
    nx_cells: &u32,
    ny_cells: &u32,
    cell_width: &u32,
) {
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    for row in 0..*ny_cells {
        for column in 0..*nx_cells {
            display_cell(renderer, row, column, &grid, &cell_width, &ant)
        }
    }
    renderer.present();
}