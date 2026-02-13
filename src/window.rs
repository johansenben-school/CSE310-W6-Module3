#![allow(non_snake_case)]

extern crate sdl2; 
use std::time::Duration;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;


use crate::board::SelectedCell;
use crate::logic;
use crate::renderer;
use crate::board;
use crate::sidebar::{SideBar};
use crate::util::*;
use crate::cell;

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
      self.renderer.clearWindow((255, 255, 255, 255));
      self.board.render(&mut self.renderer);
      self.sideBar.render(&mut self.renderer);
      self.renderer.present();
      self.rerender = false;
    }
  }
  fn handleClick(&mut self, mouseBtn: MouseButton, x: i32, y: i32) {
    match mouseBtn {
      MouseButton::Left => {
        match (x, y) {
          (xVal, yVal) if BOARD_X_RANGE.contains(&xVal) && BOARD_Y_RANGE.contains(&yVal)  => {
            self.lastEvent = self.board.handleClick(x - BOARD_X, y - BOARD_Y);
          },
          (xVal, yVal) if SIDE_BAR_X_RANGE.contains(&xVal) => {
            self.lastEvent = self.sideBar.handleClick(x - SIDE_BAR_X, y - SIDE_BAR_Y);
          }
          _ => {}
        }
      },
      _ => {}
    }

  }
  pub fn update(&mut self) {
    match self.lastEvent {
      SudokuEvent::SelectCell { index } => {
        self.board.selectCell(index);
      },
      SudokuEvent::UnselectCell => {
        self.board.unselectCell();
      },
      SudokuEvent::NumBtn{ value } => {
        if let SelectedCell::Cell(index) = self.board.cellSelected {
          if !self.board.cells[index as usize].canUserChange() {
            return;
          }
          let validNumsFuncs = [ logic::getValidNums::inRow, logic::getValidNums::inCol, logic::getValidNums::inBox ];
          let mut validNums: Vec<u8> = vec![1,2,3,4,5,6,7,8,9];
          logic::getValidNums::fromDispatch(&self.board.cells, index as i8, &mut validNums, &validNumsFuncs);
          if validNums.contains(&value) {
            self.board.setCell(value, cell::CellState::USER_INPUT);
          } else {
            self.board.setCell(value, cell::CellState::INCORRECT);
          }
        }
        self.board.unselectCell();
      },
      SudokuEvent::SolveBtn => {
        logic::solve(&mut self.board.cells);
      },
      SudokuEvent::EraseBtn => {
        if let SelectedCell::Cell(index) = self.board.cellSelected {
          if self.board.cells[index as usize].canUserChange() {
            self.board.setCell(0, cell::CellState::EMPTY);
          }
        }
        self.board.unselectCell();
      },
      SudokuEvent::ResetBtn => {
        self.board.reset();
      },
      _ => {}
    }
    self.lastEvent = SudokuEvent::None;
  }
  pub fn start(&mut self) -> Result<(), String> {
    let mut event_pump = self.renderer.getEventPump()?;
    'running: loop {
      for event in event_pump.poll_iter() {
        match event {
          Event::Quit { .. }
          | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
          } => break 'running,
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