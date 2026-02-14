#![allow(non_snake_case)]

extern crate rand;
use crate::renderer;
use crate::cell;
use crate::util::*;
use crate::logic;
use rand::seq::SliceRandom;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SelectedCell {
  None,
  Cell(u8)
}

pub struct Board {
  pub cells: [cell::Cell; 81],
  pub cellsSelected: Vec<u8>
}

impl Board {
  pub fn new() -> Board {
    let empty = cell::Cell::newEmpty();
    let mut board = Board {
      cells: [empty; 81],
      cellsSelected: vec![]
    };
    board.reset();

    return board;
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
      if self.cellsSelected.contains(&((cellY * 9 + cellX) as u8)) {
        return SudokuEvent::UnselectCell{ index: (cellY * 9 + cellX) as u8 };
      } else {
        return SudokuEvent::SelectCell { index: (cellY * 9 + cellX) as u8 }
      }
    }
    return SudokuEvent::None;
  }
  pub fn unselectCells(&mut self) {
    self.cellsSelected.clear();
  }
  pub fn unselectCell(&mut self, index: &u8) {
    self.cellsSelected.retain(|val| val != index);
  }
  pub fn selectCell(&mut self, index: u8) {
    self.cellsSelected.push(index);
  }
  pub fn userSetCells(&mut self, value: u8) {
    let validNumsFuncs = [ logic::getValidNums::inRow, logic::getValidNums::inCol, logic::getValidNums::inBox ];
    for index in &self.cellsSelected {
      if !self.cells[*index as usize].canUserChange() {
        continue;
      }
      let mut validNums: Vec<u8> = vec![1,2,3,4,5,6,7,8,9];
      logic::getValidNums::fromDispatch(&self.cells, *index as i8, &mut validNums, &validNumsFuncs);
      if value == 0 {
        self.cells[*index as usize].setCell(value, cell::CellState::EMPTY);
      } else if validNums.contains(&value) {
        self.cells[*index as usize].setCell(value, cell::CellState::USER_INPUT);
      } else {
        self.cells[*index as usize].setCell(value, cell::CellState::INCORRECT);
      }
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
        if self.cellsSelected.contains(&(y * 9 + x)) {
          renderer.renderFillRect(cell::SELECTED_CELL, cellX, cellY, CELL_WIDTH as u32, CELL_WIDTH as u32);
        }
        let color: (u8,u8,u8,u8) = match self.cells[(y * 9 + x) as usize].state {
          cell::CellState::INCORRECT => (255,0,0,255),
          cell::CellState::USER_INPUT => (0,0,255,255),
          cell::CellState::SOLVER_INPUT | cell::CellState::SOLVER_INPUT_LOCKED => (0,180,0,255),
          _ => (0,0,0,255)
        };
        renderer.renderCenteredText(self.cells[(y * 9 + x) as usize].getDisplayVal().as_str(), color, cellX, cellY, CELL_WIDTH, CELL_WIDTH);
      }
    }
  }
  pub fn reset(&mut self) {
    let mut rng = rand::thread_rng();
    let puzzle = PUZZLES.choose(&mut rng).unwrap_or(&[0;81]);
    for i in 0..81_u8 {
      self.cells[i as usize].setCell(puzzle[i as usize], if puzzle[i as usize] != 0 { cell::CellState::LOCKED } else { cell::CellState::EMPTY });
    }
    self.unselectCells();
  }

}