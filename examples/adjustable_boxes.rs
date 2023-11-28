extern crate rand;
extern crate sdl2;

use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use untitled_sdl_engine::render::window_context;

const GRID_ROWS: i32 = 80;
const GRID_COLS: i32 = 60;

fn main() {
    // Initialize SDL2
    let mut window_context = untitled_sdl_engine::render::window_context::WindowContext::new(
        None,
        "Adjustable Box".to_string(),
        800,
        600,
        false,
    );

    // Generate random colors for each square
    let colors: Vec<Color> = (0..GRID_ROWS * GRID_COLS)
        .map(|_| Color::RGB(random_color(), random_color(), random_color()))
        .collect();

    // Main loop
    'running: loop {
        for event in window_context.poll_events() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Clear the screen
        window_context.clear();

        // Draw the dynamic grid of colored squares
        draw_dynamic_grid(&mut window_context.canvas, &colors);

        // Present the canvas
        window_context.present();

        // Cap the frame rate
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_dynamic_grid(canvas: &mut Canvas<Window>, colors: &[Color]) {
    // Get the dimensions of the window
    let (window_width, window_height) = canvas.output_size().unwrap();

    // Calculate the size of each square based on the window dimensions
    let square_width = window_width as i32 / GRID_COLS;
    let square_height = window_height as i32 / GRID_ROWS;

    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            // Get the color for the current square
            let color = colors[(row * GRID_COLS + col) as usize];

            // Calculate square position
            let x = col as i32 * square_width;
            let y = row as i32 * square_height;

            // Draw the square
            canvas.set_draw_color(color);
            canvas
                .fill_rect(Rect::new(x, y, square_width as u32, square_height as u32))
                .unwrap();
        }
    }
}

fn random_color() -> u8 {
    rand::thread_rng().gen_range(0..255)
}
