extern crate piston_window;
#[macro_use] extern crate conrod_core;
extern crate conrod_piston;

use piston_window::{WindowSettings, PistonWindow};
use piston_window::{Event, Input, Loop};
use piston_window::TextureSettings;
use piston_window::Glyphs;
use piston_window::Transformed;
use piston_window::{clear, text};
use piston_window::OpenGL;
use piston_window::{G2d, G2dTexture};
use piston_window::texture::UpdateTexture;

use conrod_core::{Ui, UiBuilder};
use conrod_core::widget::{Widget, TextBox};
use conrod_core::text::GlyphCache;
use conrod_core::text::rt::Rect;
use conrod_core::image::Map;
use conrod_piston::event::convert;
use conrod_core::{Position, Positionable, Sizeable};

const OPENGL_VERSION: OpenGL = OpenGL::V4_5;
const FONT_PATH: &'static str = "./assets/fonts/GenSenMaruGothicTW-Regular.ttf";

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

widget_ids! {
    struct WidgetIds {
        text_box
    }
}

struct DrawingComponents<'font> {
    pub text_texture_cache: G2dTexture,
    pub glyph_cache: GlyphCache<'font>,
    pub image_map: Map<G2dTexture>
}

impl <'font> DrawingComponents<'font> {
    pub fn new(window: &mut PistonWindow) -> DrawingComponents<'font> {
        DrawingComponents {
            text_texture_cache: DrawingComponents::new_text_txture_cache(window),
            glyph_cache: DrawingComponents::new_glyph_cache(),
            image_map: Map::new()
        }
    }

    fn new_glyph_cache() -> GlyphCache<'font> {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;

        GlyphCache::builder()
            .dimensions(WIDTH, HEIGHT)
            .scale_tolerance(SCALE_TOLERANCE)
            .position_tolerance(POSITION_TOLERANCE)
            .build()
    }

    fn new_text_txture_cache(window: &mut PistonWindow) -> G2dTexture {
        let buffer_len = WIDTH as usize * HEIGHT as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let factory = &mut window.factory;

        G2dTexture::from_memory_alpha(factory, &init, WIDTH, HEIGHT, &settings).unwrap()
    }

    // pub fn cache_queued_glyphs(graphics: &mut G2d, cache: &mut G2dTexture, rect: Rect<u32>, data: &[u8]) {
    //     let offset = [rect.min.x, rect.min.y];
    //     let size = [rect.width(), rect.height()];
    //     let format = piston_window::texture::Format::Rgba8;
    //     let encoder = &mut graphics.encoder;
    //     text_vertex_data.clear();
    //     text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
    //     UpdateTexture::update(cache, encoder, format, &text_vertex_data[..], offset, size)
    //         .expect("failed to update texture")s
    // }
}

fn main() {
    // Build a Piston window
    let mut window: PistonWindow = 
        WindowSettings::new("Test Window", (WIDTH, HEIGHT))
        .exit_on_esc(true)
        .vsync(true)
        .opengl(OPENGL_VERSION)
        .build()
        .expect("fail to create a window");
    
    // Construct Conrod UI
    let mut ui = UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Register Widget Ids
    let ids = WidgetIds::new(ui.widget_id_generator());

    // Create neccessary caches for drawing
    let mut drawing_coms = DrawingComponents::new(&mut window);
    let mut text_vertex_data = Vec::new();

    // Create a GlyphCache (to store a font)
    // let mut glyph_cache = Glyphs::new(FONT_PATH, window.factory.clone(), TextureSettings::new())
    //     .expect("fail to load the font");

    // Event loop
    while let Some(event) = window.next() {
        // Convert the Piston event to Conrod event
        if let Some(conrod_event) = convert(event.clone(), WIDTH as f64, HEIGHT as f64) {
            ui.handle_event(conrod_event);
        }

        match event {
            Event::Input(Input::Close(_)) => {
                println!("the window is closing");
                break;
            },
            Event::Loop(Loop::Update(_)) => {
                setup_gui(&mut ui, &ids);
            },
            Event::Loop(Loop::Render(_)) => {
                // Render the screen
                window.draw_2d(&event, |context, gl| {
                    // Clear the screen
                    clear([1.0, 1.0, 1.0, 1.0], gl);

                    // Draw text
                    // text([1.0, 1.0, 1.0, 1.0], 32, "源泉圓體：測試貓貓、Alpha、Delta", 
                    //     &mut glyph_cache,
                    //     context.trans(300.0, 400.0).transform, gl)
                    //     .expect("drawing text fails");

                    // Draw the ui
                    let primitives = ui.draw(); {
                        // A function used for caching glyphs to the texture cache.
                        let cache_queued_glyphs = |graphics: &mut G2d,
                                                cache: &mut G2dTexture,
                                                rect: conrod_core::text::rt::Rect<u32>,
                                                data: &[u8]|
                            {
                                let offset = [rect.min.x, rect.min.y];
                                let size = [rect.width(), rect.height()];
                                let format = piston_window::texture::Format::Rgba8;
                                let encoder = &mut graphics.encoder;
                                text_vertex_data.clear();
                                text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                                UpdateTexture::update(cache, encoder, format, &text_vertex_data[..], offset, size)
                                    .expect("failed to update texture")
                            };

                        // Specify how to get the drawable texture from the image. In this case, the image
                        // *is* the texture.
                        fn texture_from_image<T>(img: &T) -> &T { img }

                        // Draw the conrod `render::Primitives`.
                        conrod_piston::draw::primitives(primitives, context, gl,
                            &mut drawing_coms.text_texture_cache,
                            &mut drawing_coms.glyph_cache,
                            &drawing_coms.image_map,
                            cache_queued_glyphs, texture_from_image);
                    }
                });
            },
            _ => {}
        }
    }
}

fn setup_gui(ui: &mut Ui, ids: &WidgetIds) {
    let mut ui_cell = ui.set_widgets();

    // TextBox
    TextBox::new("Edit").x(500.0).y(500.0).w(100.0).h(20.0)
    .set(ids.text_box, &mut ui_cell);
}