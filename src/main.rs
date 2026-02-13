extern crate sdl2; 

mod window;
pub mod util;
pub mod renderer;
pub mod board;
pub mod cell;
pub mod logic;
pub mod sidebar;

fn main() -> Result<(), String> {

    let ttf_context = sdl2::ttf::init().unwrap();
    let mut window = window::Window::new(&ttf_context);
    
    _ = window.start();

    Ok(())
}
