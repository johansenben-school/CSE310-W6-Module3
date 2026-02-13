#![allow(non_snake_case)]
use crate::util::*;
use crate::renderer::{Renderer};

pub struct SideBar {
  numBtnCoords: [(i32, i32); 9],
  solveBtn: (i32, i32, i32, i32),
  eraseBtn: (i32, i32, i32, i32),
  resetBtn: (i32, i32, i32, i32),
  multiSelectBtn: (i32,i32,i32,i32),
  pub multiSelect: bool
}

impl SideBar {
  pub fn new() -> Self {
    SideBar {
      numBtnCoords: [ 
      (0, 0), (110, 0), (220, 0),
      (0, 110), (110, 110), (220, 110),
      (0, 220), (110, 220), (220, 220)
     ], 
     solveBtn: (0, 440, 320, 100),
     eraseBtn: (0, 330, 100, 100),
     resetBtn: (110, 330, 100, 100),
     multiSelectBtn: (220, 330, 100, 100),
     multiSelect: false
    }
  }

  fn renderTextBtn(renderer: &mut Renderer, text: &str, rect: (i32, i32, i32, i32)) {
    renderer.renderRect((0,0,0,255), SIDE_BAR_X + rect.0, SIDE_BAR_Y + rect.1, rect.2 as u32, rect.3 as u32);
    renderer.renderCenteredText(text, (0,0,0,255), SIDE_BAR_X + rect.0, SIDE_BAR_Y + rect.1, rect.2, rect.3);
  }
  fn isInRange(x: i32, y: i32, rect: (i32, i32, i32, i32)) -> bool{
    (rect.0..(rect.0 + rect.2)).contains(&x) && (rect.1..(rect.1 + rect.3)).contains(&y)
  }

  pub fn render(&self, renderer: &mut Renderer) {
     for i in 0..9 {
      Self::renderTextBtn(renderer, (i + 1).to_string().as_str(), (self.numBtnCoords[i].0, self.numBtnCoords[i].1, CELL_WIDTH, CELL_WIDTH));
     }

     Self::renderTextBtn(renderer, "Solve", self.solveBtn);
     Self::renderTextBtn(renderer, "✖", self.eraseBtn);
     Self::renderTextBtn(renderer, "↻", self.resetBtn);
     Self::renderTextBtn(renderer, if self.multiSelect { "☐☐" } else { "☐" }, self.multiSelectBtn);
  }

  pub fn handleClick(&mut self, x: i32, y: i32) -> SudokuEvent {
    for i in 0..9 {
      let btnX = self.numBtnCoords[i].0;
      let btnY = self.numBtnCoords[i].1;
      if (btnX..(btnX + CELL_WIDTH)).contains(&x) && (btnY..(btnY + CELL_WIDTH)).contains(&y) {
        println!("Button clicked: {}", i + 1);
        return SudokuEvent::NumBtn {value: (i + 1) as u8};
      }
     }
     match (x, y) {
      (xVal, yVal) if Self::isInRange(xVal, yVal, self.solveBtn) => {
        return SudokuEvent::SolveBtn;
      },
      (xVal, yVal) if Self::isInRange(xVal, yVal, self.eraseBtn) => {
        return SudokuEvent::EraseBtn;
      },
      (xVal, yVal) if Self::isInRange(xVal, yVal, self.resetBtn) => {
        return SudokuEvent::ResetBtn;
      },
      (xVal, yVal) if Self::isInRange(xVal, yVal, self.multiSelectBtn) => {
        self.multiSelect = !self.multiSelect;
        return SudokuEvent::MultiSelectBtn;
      },
      _ => {
        return SudokuEvent::None;
      }
     }
  }
}