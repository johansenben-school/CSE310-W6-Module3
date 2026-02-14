#![allow(non_snake_case)]

extern crate sdl2; 
use std::time::Duration;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;


use crate::logic;
use crate::renderer;
use crate::board;
use crate::sidebar::{SideBar};
use crate::util::*;

pub struct Window<'a> {
  renderer: renderer::Renderer<'a>,
  board: board::Board,
  rerender: bool,
  sideBar: SideBar,
  lastEvent: SudokuEvent //makes it easier to add an event queue
}


impl<'a> Window<'a> {
  pub fn new(ttfContext: &'a sdl2::ttf::Sdl2TtfContext) -> Self {
    let renderer = renderer::Renderer::new(&ttfContext);
    let board = board::Board::new();

    Window {
      renderer,
      board,
      rerender: true,
      sideBar: SideBar::new(),
      lastEvent: SudokuEvent::None
    }
  }
  
  fn render(&mut self) {
    if self.rerender {
      //clear
      self.renderer.clearWindow((255, 255, 255, 255));

      //render
      self.board.render(&mut self.renderer);
      self.sideBar.render(&mut self.renderer);

      //render present
      self.renderer.present();

      //dont rerender until an update happens
      self.rerender = false;
    }
  }

  fn handleClick(&mut self, mouseBtn: MouseButton, x: i32, y: i32) {
    match mouseBtn {
      MouseButton::Left => {
        match (x, y) {
          //board clicked
          (xVal, yVal) if BOARD_X_RANGE.contains(&xVal) && BOARD_Y_RANGE.contains(&yVal)  => {
            self.lastEvent = self.board.handleClick(x - BOARD_X, y - BOARD_Y);
          },
          //sidebar clicked
          (xVal, yVal) if SIDE_BAR_X_RANGE.contains(&xVal) => {
            self.lastEvent = self.sideBar.handleClick(x - SIDE_BAR_X, y - SIDE_BAR_Y);
          }
          _ => {
            //if anywhere else was clicked -> unselect all cells
            self.lastEvent = SudokuEvent::UnselectAll;
          }
        }
      },
      _ => {}
    }
  }
  
  pub fn update(&mut self) {
    match self.lastEvent { //which button was clicked last? + other events
      //select a cell
      SudokuEvent::SelectCell { index } => {
        if !self.sideBar.multiSelect {
          self.board.unselectCells();
        }
        self.board.selectCell(index);
      },
      //unselect a cell
      SudokuEvent::UnselectCell { index } => {
        self.board.unselectCell(&index);
      },
      //number button on the sidebar was clicked
      SudokuEvent::NumBtn{ value } => {
        self.board.userSetCells(value);
        self.board.unselectCells();
      },
      //solve button clicked
      SudokuEvent::SolveBtn => {
        logic::solve(&mut self.board.cells);
      },
      //erase button clicked
      SudokuEvent::EraseBtn => {
        self.board.userSetCells(0);
        self.board.unselectCells();
      },
      //reset button clicked
      SudokuEvent::ResetBtn => {
        self.board.reset();
      },
      //multi select button clicked (multi-select state is stored in sidebar)
      SudokuEvent::MultiSelectBtn => {
        self.board.unselectCells();
      },
      //unselect every cell
      SudokuEvent::UnselectAll => {
        self.board.unselectCells();
      },
      _ => {}
    }
    self.lastEvent = SudokuEvent::None;
  }

  pub fn start(&mut self) -> Result<(), String> {
    let mut event_pump = self.renderer.getEventPump()?;
    'running: loop {

      //events
      for event in event_pump.poll_iter() {
        match event {

          //quit
          Event::Quit { .. }
          | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
          } => break 'running,

          //click
          Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn, clicks: _, x, y } => {
            self.handleClick(mouse_btn, x, y);
            self.rerender = true;
          },
          _ => {}
        }
      }
      self.update();
      self.render();

      // Sleep a little to not burn CPU
      std::thread::sleep(Duration::from_millis(20));
    }
    Ok(())
  }
}