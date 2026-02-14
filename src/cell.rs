#![allow(non_snake_case)]

use crate::util::*;

#[derive(PartialEq, Eq, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CellState {
  EMPTY, LOCKED, INCORRECT, SOLVER_INPUT, USER_INPUT, SOLVER_INPUT_LOCKED
}
#[derive(Copy, Clone)]
pub struct Cell {
  val: u8,
  pub state: CellState
}
pub const SELECTED_CELL: Color = (100,150,255,255);
pub const SAME_ROW_CELL: Color = (200,200,200,255);
pub const SAME_COL_CELL: Color = (200,200,200,255);


impl Cell {
  pub fn new(val: u8, state: CellState) -> Cell {
    Cell { val, state }
  }
  pub fn newEmpty() -> Self {
    Self { val: 0, state: CellState::EMPTY }
  }
  pub fn makeEmpty(&mut self) {
    self.state = CellState::EMPTY;
    self.val = 0;
  }
  
  pub fn getDisplayVal(&mut self) -> String {
    if self.state == CellState::EMPTY {
      " ".to_string()
    } else {
      self.val.to_string()
    }
  }
  pub fn getVal(self) -> u8 {
    if [CellState::EMPTY, CellState::INCORRECT].contains(&self.state) {
      0
    } else {
      self.val
    }
  }
  pub fn isEmpty(self) -> bool {
    [CellState::EMPTY, CellState::INCORRECT].contains(&self.state)
  }
  pub fn isLocked(self) -> bool {
    self.state == CellState::LOCKED
  }
  pub fn canSolverChange(self) -> bool {
    !self.isLocked() && self.state != CellState::USER_INPUT && self.state != CellState::SOLVER_INPUT_LOCKED
  }
  pub fn canUserChange(self) -> bool {
    !self.isLocked() && self.state != CellState::SOLVER_INPUT
  }
  pub fn setCell(&mut self, val: u8, state: CellState) {
    self.val = val;
    self.state = state;
  }
}