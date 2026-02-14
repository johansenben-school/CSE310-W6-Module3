#![allow(non_snake_case)]

use crate::{cell::{Cell, CellState}, logic::getValidNums::placeEasyCells};
pub mod getValidNums {
    use crate::cell::{Cell, CellState};


  pub fn inRow(board: &[Cell; 81], index: i8, validNums: &mut Vec<u8>) {
    let y: i8 = index / 9;
    for x in 0..9 {
      let i: i8 = y * 9 + x;
      if i == index {
        continue;
      }
      let cell: Cell = board[i as usize];
      if !cell.isEmpty() {
        validNums.retain(|&val| val != cell.getVal());
      }
    }
  }

  pub fn inCol(board: &[Cell; 81], index: i8, validNums: &mut Vec<u8>) {
    let x: i8 = index % 9;
    for y in 0..9 {
      let i: i8 = y * 9 + x;
      if i == index {
        continue;
      }
      let cell: Cell = board[i as usize];
      if !cell.isEmpty() {
        validNums.retain(|&val| val != cell.getVal());
      }
    }
  }

  pub fn inBox(board: &[Cell; 81], index: i8, validNums: &mut Vec<u8>) {
    let boxX = (index % 9) / 3;
    let boxY = (index / 9) / 3;
    
    for y in (boxY * 3)..(boxY * 3 + 3) {
      for x in (boxX * 3)..(boxX * 3 + 3) {
        let i: i8 = y * 9 + x;
        if i == index {
          continue;
        }
        let cell: Cell = board[i as usize];
        if !cell.isEmpty() {
          validNums.retain(|&val| val != cell.getVal());
        }
      }
    }
  }

  pub fn fromDispatch(board: &[Cell; 81], index: i8, validNums: &mut Vec<u8>, funcs: &[fn(&[Cell;81], i8, &mut Vec<u8>)]) {
    for func in funcs {
      func(board, index, validNums);
    }
  }

  pub fn placeEasyCells(board: &mut [Cell; 81], funcs: &[fn(&[Cell;81], i8, &mut Vec<u8>)]) {
    let mut changedVal;
    loop {
      changedVal = false;
      for i in 0..81_i8 {
        if !board[i as usize].canSolverChange() {
          continue;
        }
        let mut validNums: Vec<u8> = vec![1,2,3,4,5,6,7,8,9];
        fromDispatch(board, i, &mut validNums, &funcs);

        if validNums.len() == 1 {
          board[i as usize].setCell(validNums[0], CellState::SOLVER_INPUT_LOCKED);
          changedVal = true;
        }
      }

      let mut candidates: Vec<Vec<u8>> = vec![vec![]; 81];
      for i in 0..81 {
          if board[i].canSolverChange() {
              let mut valid = vec![1,2,3,4,5,6,7,8,9];
              fromDispatch(board, i as i8, &mut valid, &funcs);
              candidates[i] = valid;
          }
      }
      for rowOrCol in 0..9_u8 {
        for num in 1..=9_u8 {
          let mut countInRow: u8 = 0;
          let mut countInCol: u8 = 0;
          let mut lastIndexInRowToChange: i8 = -1;
          let mut lastIndexInColToChange: i8 = -1;
          for rowOrCol2 in 0..9_u8 {
            if candidates[(rowOrCol * 9 + rowOrCol2) as usize].contains(&num) {
              lastIndexInRowToChange = (rowOrCol * 9 + rowOrCol2) as i8;
              countInRow += 1;
            }
            if candidates[(rowOrCol2 * 9 + rowOrCol) as usize].contains(&num) {
              lastIndexInColToChange = (rowOrCol2 * 9 + rowOrCol) as i8;
              countInCol += 1;
            }
          }
          if countInRow == 1 {
            board[lastIndexInRowToChange as usize].setCell(num, CellState::SOLVER_INPUT_LOCKED);
            changedVal = true;
          }
          if countInCol == 1 {
            board[lastIndexInColToChange as usize].setCell(num, CellState::SOLVER_INPUT_LOCKED);
            changedVal = true;
          }
        }
      }

      if !changedVal {
        break;
      }
    }
    
  }
}

pub enum SolveState {
  SOLVED, UNSOLVEABLE
}
pub fn solve(board: &mut [Cell; 81]) -> SolveState {
  let mut filledCells: u8 = 0;
  for cell in &mut *board {
    if !cell.isEmpty() {
      filledCells += 1;
      if cell.state == CellState::SOLVER_INPUT {
        cell.setCell(cell.getVal(), CellState::USER_INPUT);
      }
    } 
  }
  //dont solve again if already solved
  if filledCells == 81 {
    return SolveState::SOLVED;
  }

  let validNumsFuncs = [ getValidNums::inRow, getValidNums::inCol, getValidNums::inBox ];
  let mut currentCell: i8 = 0;
  let mut validNums: Vec<u8> = vec![1,2,3,4,5,6,7,8,9];
  getValidNums::fromDispatch(board, currentCell, &mut validNums, &validNumsFuncs);
  let mut tryVal: u8 = 1;
  let mut isBackwards = false;

  placeEasyCells(board, &validNumsFuncs);

  let mut count: u32 = 0;
  while count < 500000 {
    //increment count to make sure that loop isn't infinite
    count += 1;

    if currentCell >= 81 {
      return SolveState::SOLVED
    }
    //if cell is locked, skip it
    if !board[currentCell as usize].canSolverChange() {
      //increment currentCell
      if isBackwards {
        if currentCell == 0 {
          return SolveState::UNSOLVEABLE;
        }
        currentCell -= 1;
        //set tryVal to the old value of the cell + 1
        tryVal = board[currentCell as usize].getVal() + 1;
      } else {
        currentCell += 1;
        tryVal = 1;
        if currentCell >= 81 {
          return SolveState::SOLVED
        }
      }

      //reinitialize variables for different cell
      validNums = vec![1,2,3,4,5,6,7,8,9];
      getValidNums::fromDispatch(board, currentCell, &mut validNums, &validNumsFuncs);
      continue;
    } else {
      //if not locked, assume direction is forwards
      isBackwards = false;
    }

    //if tried every value
    if tryVal > 9 {
      //reset the cell
      board[currentCell as usize].makeEmpty();

      if currentCell == 0 {
        return SolveState::UNSOLVEABLE;
      }
      //go back to the last cell
      currentCell -=1;
      //assume direction is backwards until it changes
      isBackwards = true;
      //reset variables
      tryVal = board[currentCell as usize].getVal() + 1;
      validNums = vec![1,2,3,4,5,6,7,8,9];
      getValidNums::fromDispatch(board, currentCell, &mut validNums, &validNumsFuncs);

      continue;
    }

    //if value is not valid -> try next value
    if !validNums.contains(&tryVal) {
      tryVal += 1;
      continue;
    }

    //set cell
    board[currentCell as usize].setCell(tryVal, CellState::SOLVER_INPUT);
    if currentCell >= 80 { //if the last cell got set -> it's solved
      return SolveState::SOLVED
    }

    //reintialize variables for the next cell
    currentCell += 1;
    tryVal = 1;
    isBackwards = false;
    validNums = vec![1,2,3,4,5,6,7,8,9];
    getValidNums::fromDispatch(board, currentCell, &mut validNums, &validNumsFuncs);

    
  }
  //loop exited without solving puzzle; either the puzzle is unsolveable or the solver gave up
  println!("Failed to solve");
  return SolveState::UNSOLVEABLE
}