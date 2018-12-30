extern crate piston_window;
extern crate shader_version;

use shader_version::OpenGL;
use piston_window::{WindowSettings, PistonWindow};
use piston_window::{Event, Input, Loop};
use piston_window::TextureSettings;
use piston_window::Glyphs;
use piston_window::Transformed;
use piston_window::{clear, text};

const OPENGL_VERSION: OpenGL = OpenGL::V4_5;
const FONT_PATH: &'static str = "./assets/fonts/NotoSans-Regular.ttf";

fn main() {
    // Build a window
    let mut window: PistonWindow = 
        WindowSettings::new("Test Window", (1280, 720))
        .exit_on_esc(true)
        .vsync(true)
        .opengl(OPENGL_VERSION)
        .build()
        .expect("fail to create a window");

    // Create a GlyphCache (to store a font)
    let mut glyph_cache = Glyphs::new(FONT_PATH, window.factory.clone(), TextureSettings::new())
        .expect("fail to load the font");

    // Event loop
    while let Some(event) = window.next() {
        match event {
            Event::Input(Input::Close(_)) => {
                println!("the window is closing");
                break;
            },
            Event::Loop(Loop::Render(_)) => {
                // Render the screen
                window.draw_2d(&event, |context, gl| {
                    clear([0.0, 0.0, 0.0, 1.0], gl);
                    text([1.0, 1.0, 1.0, 1.0], 32, "Test Meow", 
                        &mut glyph_cache,
                        context.trans(300.0, 400.0).transform, gl)
                        .expect("drawing text fails");
                });
            },
            _ => {}
        }
    }
}