#![allow(non_snake_case)]

extern crate sdl2; 


use crate::util::*;

pub struct Renderer<'a> {
  sdlContext: sdl2::Sdl,
  canvas: sdl2::render::Canvas<sdl2::video::Window>,
  font: sdl2::ttf::Font<'a, 'a>,
  textureCreator: sdl2::render::TextureCreator<sdl2::video::WindowContext>
}

impl<'a> Renderer<'a> {
  pub fn new(ttfContext: &'a sdl2::ttf::Sdl2TtfContext) -> Self {
    let sdlContext = sdl2::init().unwrap();
    let videoSubsystem = sdlContext.video().unwrap();

    let canvas = videoSubsystem
      .window(WIN_TITLE, WIN_WIDTH, WIN_HEIGHT)
      .position_centered()
      .build()
      .map_err(|e| e.to_string()).unwrap()
      .into_canvas()
      .build()
      .map_err(|e| e.to_string()).unwrap();

    let textureCreator = canvas.texture_creator();
    let font = ttfContext
        .load_font("Menlo.ttc", 64).unwrap();

    Renderer {
      sdlContext,
      canvas,
      font,
      textureCreator
    }
  }

  pub fn getEventPump(&mut self) -> Result<sdl2::EventPump, String> {
    let eventPump = self.sdlContext.event_pump()?;
    Ok(eventPump)
  }
  
  pub fn setColor(&mut self, color: Color) {
    self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(color.0, color.1, color.2, color.3));
  }

  pub fn clearWindow(&mut self, color: Color) {
    self.setColor(color);
    self.canvas.clear();
  }

  //fillangle rect
  pub fn renderFillRect(&mut self, color: Color, x: i32, y: i32, width: u32, height: u32) {
    self.setColor(color);
    let rect = sdl2::rect::Rect::new(x, y, width, height);
    self.canvas.fill_rect(rect).unwrap();
  }

  //rectangle with just outline
  pub fn renderRect(&mut self, color: Color, x: i32, y: i32, width: u32, height: u32) {
    self.setColor(color);
    let rect = sdl2::rect::Rect::new(x, y, width, height);
    self.canvas.draw_rect(rect).unwrap();
  }

  //text centered in a box
  pub fn renderCenteredText(&mut self, text: &str, color: Color, boxX: i32, boxY: i32, boxWidth: i32, boxHeight: i32) {
    let surface = self.font.render(text)
      .blended(color)
      .unwrap();

    let mut texture = self.textureCreator
      .create_texture_from_surface(&surface)
      .unwrap();

    let query = texture.query();
    let target = sdl2::rect::Rect::new(boxX + (boxWidth - query.width as i32) as i32 / 2, boxY as i32 + (boxHeight - query.height as i32) as i32 / 2, query.width, query.height);
    self.canvas.copy(&mut texture, None, Some(target)).unwrap();
  }
  
  pub fn present(&mut self) {
    self.canvas.present();
  }

}