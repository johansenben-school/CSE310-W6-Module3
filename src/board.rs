#![allow(non_snake_case)]

use crate::renderer;
use crate::cell;
use crate::util::*;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SelectedCell {
  None,
  Cell(u8)
}

pub struct Board {
  pub cells: [cell::Cell; 81],
  pub cellSelected: SelectedCell
}

impl Board {
  pub fn new() -> Board {
    let empty = cell::Cell::newEmpty();
    Board {
      cells: [empty; 81],
      cellSelected: SelectedCell::None
    }
  }
  pub fn handleClick(&mut self, x: i32, y: i32) -> SudokuEvent {
    let lines = [ BORDER, THIN_LINE, THIN_LINE, THICK_LINE, THIN_LINE, THIN_LINE, THICK_LINE, THIN_LINE, THIN_LINE, BORDER ];
    let mut widthUsed = 0;
    let mut cellX: i8 = -1;
    let mut cellY: i8 = -1;
    for i in 0..(lines.len() as usize) {
      widthUsed += lines[i as usize];
      if x > widthUsed && x <= CELL_WIDTH + widthUsed {
        cellX = i as i8;
      }
      if y > widthUsed && y <= CELL_WIDTH + widthUsed {
        cellY = i as i8;
      }
      widthUsed += CELL_WIDTH;
    }
    if (0..9).contains(&cellX) && (0..9).contains(&cellY) {
      println!("Clicked cell: {} = {}, {}", cellY * 9 + cellX, cellX, cellY);
      match self.cellSelected {
        SelectedCell::Cell( val) => {
          if val != (cellY * 9 + cellX) as u8 {
            return SudokuEvent::SelectCell { index: (cellY * 9 + cellX) as u8 }
          }
          return SudokuEvent::UnselectCell;
        },
        SelectedCell::None => {
          return SudokuEvent::SelectCell { index: (cellY * 9 + cellX) as u8 }
        }
      }
    }
    return SudokuEvent::None;
  }
  pub fn unselectCell(&mut self) {
    self.cellSelected = SelectedCell::None;
  }
  pub fn selectCell(&mut self, index: u8) {
    self.cellSelected = SelectedCell::Cell(index);
  }
  pub fn setCell(&mut self, value: u8, state: cell::CellState) {
    if let SelectedCell::Cell(index) = self.cellSelected {
      self.cells[index as usize].setCell(value, state);
    }
  }
  pub fn render(&mut self, renderer: &mut renderer::Renderer) {
    let lines = [ BORDER, THIN_LINE, THIN_LINE, THICK_LINE, THIN_LINE, THIN_LINE, THICK_LINE, THIN_LINE, THIN_LINE, BORDER ];
    let mut lineThicknessUsed = 0;
    for i in 0..(lines.len() as i32) {
      renderer.renderFillRect((0,0,0,255), i * CELL_WIDTH + lineThicknessUsed + BOARD_X, BOARD_Y, lines[i as usize] as u32, BOARD_WIDTH as u32);
      renderer.renderFillRect((0,0,0,255), BOARD_X, i * CELL_WIDTH + lineThicknessUsed + BOARD_Y, BOARD_WIDTH as u32, lines[i as usize] as u32);
      lineThicknessUsed += lines[i as usize];
    }
    
    let mut lineThicknessUsedY = 0;
    for y in 0..((lines.len() - 1) as u8) {
      lineThicknessUsedY += lines[y as usize];
      let mut lineThicknessUsedX = 0;
      for x in 0..((lines.len() - 1) as u8) {
        lineThicknessUsedX += lines[x as usize];
        let cellX = x as i32 * CELL_WIDTH + lineThicknessUsedX + BOARD_X;
        let cellY = y as i32 * CELL_WIDTH + lineThicknessUsedY + BOARD_Y;
        match self.cellSelected {
          SelectedCell::Cell(index) if index == y * 9 + x => {
            renderer.renderFillRect(cell::SELECTED_CELL, cellX, cellY, CELL_WIDTH as u32, CELL_WIDTH as u32);
          },
          SelectedCell::Cell(index) if x == index % 9 => {
            renderer.renderFillRect(cell::SAME_COL_CELL, cellX, cellY, CELL_WIDTH as u32, CELL_WIDTH as u32);
          },
          SelectedCell::Cell(index) if y == index / 9 => {
            renderer.renderFillRect(cell::SAME_ROW_CELL, cellX, cellY, CELL_WIDTH as u32, CELL_WIDTH as u32);
          },
          _ => {}
        }
        let color: (u8,u8,u8,u8) = match self.cells[(y * 9 + x) as usize].state {
          cell::CellState::INCORRECT => (255,0,0,255),
          cell::CellState::USER_INPUT => (0,0,255,255),
          cell::CellState::SOLVER_INPUT => (0,180,0,255),
          _ => (0,0,0,255)
        };
        renderer.renderCenteredText(self.cells[(y * 9 + x) as usize].getDisplayVal().as_str(), color, cellX, cellY, CELL_WIDTH, CELL_WIDTH);
      }
    }
  }
  pub fn reset(&mut self) {
    for cell in &mut self.cells {
      cell.setCell(0, cell::CellState::EMPTY);
    }
    self.cellSelected = SelectedCell::None;
  }

}