extern crate piston_window;

use piston_window::{Window, WindowSettings, PistonWindow};
use piston_window::{Event, Input};

fn main() {
    // Build a window
    let mut window: PistonWindow = 
        WindowSettings::new("Test Window", (1280, 720))
        .exit_on_esc(true)
        .build()
        .expect("fail to create a window");

    // Event loop
    while let Some(event) = window.next() {
        
        // Method 1
        if window.should_close() {
            println!("the window is closing");
            break;
        }
        
        // Method 2
        // match event {
        //     Event::Input(Input::Close(_)) => {
        //         println!("the window is closing");
        //         break;
        //     },
        //     _ => {}
        // }
    }
}