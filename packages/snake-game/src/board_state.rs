enum PositionType {
  Food,
  Snake,
}

// @note This will be used as an intermediary for now. Eventually when the math
//       is figured out, this should be the primary state holder of the board and
//       we should impl IntoSnakeGame for this, to get an easier-to-work-with
//       type.
pub struct BoardState {
  board: Vec<Option<PositionType>>
}

pub trait IntoBoardState {
  fn convert_to_board_state(&mut self) -> BoardState;
}