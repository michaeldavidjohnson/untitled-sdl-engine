use gl;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

pub struct WindowContext {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub sdl_context: Sdl,
    pub ttf_context: Sdl2TtfContext,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    font_path: Option<String>,
}

impl WindowContext {
    pub fn new(
        font_path: Option<&str>,
        window_title: String,
        window_width: u32,
        window_height: u32,
        use_open_gl: bool,
    ) -> Self {
        let sdd_context = sdl2::init().unwrap();
        let video_subsystem = sdd_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();
        let event_pump = sdd_context.event_pump().unwrap();
        let window: Window;
        //change the window to have openGL backend, presumably we also need
        //to get the openGL context.
        if use_open_gl {
            window = video_subsystem
                .window(&window_title, window_width.clone(), window_height.clone())
                .position_centered()
                .opengl()
                .resizable()
                .build()
                .unwrap();
        } else {
            window = video_subsystem
                .window(&window_title, window_width.clone(), window_height.clone())
                .position_centered()
                .resizable()
                .build()
                .unwrap();
        }

        let canvas = window.into_canvas().build().unwrap();

        match font_path {
            Some(font) => {
                return WindowContext {
                    sdl_context: sdd_context,
                    window_title: window_title,
                    window_height: window_height,
                    window_width: window_width,
                    event_pump: event_pump,
                    canvas: canvas,
                    ttf_context: ttf_context,
                    font_path: Some(font.to_string()),
                }
            }
            None => {
                return WindowContext {
                    sdl_context: sdd_context,
                    window_title: window_title,
                    window_height: window_height,
                    window_width: window_width,
                    event_pump: event_pump,
                    canvas: canvas,
                    ttf_context: ttf_context,
                    font_path: None,
                }
            }
        }
    }

    pub fn render_text(&mut self, text: &str, x: i32, y: i32) {
        if self.font_path.is_none() {
            return;
        } else {
            let font = self
                .ttf_context
                .load_font(&self.font_path.as_ref().unwrap(), 36)
                .unwrap();
            let surface = font.render(text).blended(Color::RGB(0, 0, 0)).unwrap();
            let texture_creator = self.canvas.texture_creator();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            let target_rect = Rect::new(x, y, surface.width(), surface.height());
            self.canvas.copy(&texture, None, target_rect).unwrap();
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        self.event_pump.poll_iter().collect()
    }
}
