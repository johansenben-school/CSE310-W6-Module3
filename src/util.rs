
pub type Color = (u8, u8, u8, u8);

//board dimensions
pub const BOARD_X: i32 = 20;
pub const BOARD_Y: i32 = 20;
pub const BORDER: i32 = 6;
pub const THICK_LINE: i32 = 4;
pub const THIN_LINE: i32 = 1;
pub const CELL_WIDTH: i32 = 80;
pub const BOARD_WIDTH: i32 = CELL_WIDTH * 9 + BORDER * 2 + THICK_LINE * 2 + THIN_LINE * 6;
pub const BOARD_X_RANGE: std::ops::Range<i32> = BOARD_X..(BOARD_X + BOARD_WIDTH);
pub const BOARD_Y_RANGE: std::ops::Range<i32> = BOARD_Y..(BOARD_Y + BOARD_WIDTH);

//sidebar dimensions
pub const SIDE_BAR_WIDTH: i32 = CELL_WIDTH * 3 + 20;
pub const SIDE_BAR_PADDING: i32 = 10;
pub const SIDE_BAR_X: i32 = BOARD_X + BOARD_WIDTH + SIDE_BAR_PADDING;
pub const SIDE_BAR_Y: i32 = BOARD_X;
pub const SIDE_BAR_X_RANGE: std::ops::Range<i32> = SIDE_BAR_X..(SIDE_BAR_X + SIDE_BAR_WIDTH);

//window dimensions
pub const WIN_WIDTH: u32 = (BOARD_WIDTH + 20 * 2 + SIDE_BAR_WIDTH + SIDE_BAR_PADDING * 2) as u32;
pub const WIN_HEIGHT: u32 = (BOARD_WIDTH + 20 * 2) as u32;
pub const WIN_TITLE: &str = "Sudoku";

//events
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SudokuEvent {
  None, NumBtn{ value: u8}, SolveBtn, CheckBtn, EraseBtn, MultiSelectBtn, UnselectCell{ index: u8 }, SelectCell{ index: u8 }, ButtonsLocked, ResetBtn, UnselectAll
}

//puzzles
pub const PUZZLES: [[u8;81];2] = [
  [
    0,0,0,0,0,3,0,0,0,
    0,0,0,6,9,7,0,4,1,
    9,0,4,8,0,0,0,0,0,
    4,9,6,5,0,0,0,0,0,
    7,0,2,9,8,6,0,0,0,
    0,5,8,0,2,0,1,0,9,
    2,0,3,1,0,0,7,8,0,
    5,8,0,0,6,0,0,0,0,
    0,6,9,0,7,8,4,5,0
  ],
  [
    0,0,0,0,0,1,6,0,0,
    9,0,0,0,0,0,0,0,0,
    0,7,8,0,0,0,0,2,5,
    8,0,0,0,0,0,0,0,0,
    0,0,0,0,0,6,0,5,9,
    3,0,0,9,5,0,0,0,0,
    0,0,0,0,0,0,0,0,3,
    6,0,0,8,0,0,4,1,0,
    0,4,5,7,0,0,0,0,0
  ]
];