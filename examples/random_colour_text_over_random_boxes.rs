extern crate rand;
extern crate sdl2;

use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf;
use sdl2::video::Window;
use untitled_sdl_engine::render::window_context;

const GRID_ROWS: i32 = 80;
const GRID_COLS: i32 = 60;
const FONT_SIZE: u16 = 10;

fn main() {
    let mut window_context = untitled_sdl_engine::render::window_context::WindowContext::new(
        Some("../resources/square.ttf"),
        "Adjustable Box".to_string(),
        800,
        600,
        true,
    );

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

        // Generate random colors for each square
        let colors: Vec<Color> = (0..GRID_ROWS * GRID_COLS)
            .map(|_| Color::RGB(random_color(), random_color(), random_color()))
            .collect();

        // Generate random text for each square
        let texts: Vec<String> = (0..GRID_ROWS * GRID_COLS).map(|_| random_text()).collect();

        // Clear the screen
        window_context.clear();

        // Draw the dynamic grid of colored squares with text
        draw_dynamic_grid(
            &mut window_context.canvas,
            &colors,
            &texts,
            &window_context
                .ttf_context
                .load_font("./resources/square.ttf", 10)
                .unwrap(),
        );

        // Present the canvas
        window_context.present();

        // Cap the frame rate
    }
}

fn draw_dynamic_grid(
    canvas: &mut Canvas<sdl2::video::Window>,
    colors: &[Color],
    texts: &[String],
    font: &ttf::Font,
) {
    // Get the dimensions of the window
    let (window_width, window_height) = canvas.output_size().unwrap();

    // Calculate the size of each square based on the window dimensions
    let square_width = window_width as i32 / GRID_COLS;
    let square_height = window_height as i32 / GRID_ROWS;

    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            // Get the color for the current square
            let color = colors[(row * GRID_COLS + col) as usize];

            // Get the text for the current square
            let text = &texts[(row * GRID_COLS + col) as usize];

            // Calculate square position
            let x = col as i32 * square_width;
            let y = row as i32 * square_height;

            // Draw the square
            canvas.set_draw_color(color);
            canvas
                .fill_rect(Rect::new(x, y, square_width as u32, square_height as u32))
                .unwrap();

            // Render and draw the text
            draw_text(canvas, text, font, x, y, square_width, square_height);
        }
    }
}

fn draw_text(
    canvas: &mut Canvas<sdl2::video::Window>,
    text: &str,
    font: &ttf::Font,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    let color = Color::RGB(random_color(), random_color(), random_color());

    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())
        .unwrap();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
        .unwrap();

    let target_rect = Rect::new(x, y, width as u32, height as u32);
    canvas.copy(&texture, None, target_rect).unwrap();
}

fn random_color() -> u8 {
    rand::thread_rng().gen_range(0..255)
}

fn random_text() -> String {
    let length = 1;
    (0..length)
        .map(|_| char::from(rand::thread_rng().gen_range(b'A'..=b'Z')))
        .collect()
}
