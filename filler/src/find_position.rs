use std::collections::VecDeque;

use crate::{find_::{Finder, Compas}, parse_::Parser};

impl Finder {
  /** todo: find the optimal position for the piece
   * iterate all possible positions for the piece and in time of iteration:
   * 
  * find the most agressively placed enemy cell, opposite the major direction
  * (placed on the player territory, or aimed closer to the player territory)
  *
  * find the most agressively placed player cell, on the major direction
  * (placed on the enemy territory, or aimed closer to the enemy territory)
  *
  * //todo: refactor bottom comments, to possible one iteration for all, with save, probably two possible answers, before choose the best one
  * find all possible variants to place the piece on the field correctly
  *
  * iterate the correct variants
  * check if there is variant to place the piece on the field that some cell of that piece
  * will be placed not the less agressively than the enemy cell, so the same or more agressively
  * from the enemy position. if there is such variant - place the piece there(set the self.answer)
  * to try prevent or restrict the enemy cell to move in the major direction
  *
  * otherwise iterate the correct variants again
  * and place the piece on the field with the most agressively placed player cell, as possible deep
  * in the major direction, to try cover more enemy cells in the major direction
  *
  * otherwise return the surrender answer (the most far enemy position)
  */
  pub fn find_position(&mut self, parser: &mut Parser) -> [usize;2] {
    let anfield = &parser.anfield;
    let piece = &parser.piece;
    let player = &parser.player_char;

    /** the piece position */
    let mut answer_xy = [usize::MAX, usize::MAX];
    /** the most argessive enemy cell position */
    let mut enemy_xy = [usize::MAX, usize::MAX];
    /** the most argessive player cell position */
    let mut player_xy = [usize::MAX, usize::MAX];

    let piece_height = piece.len();
    let piece_width = piece[0].len();

    for y in 0..anfield.len() - piece_height + 1 {
      for x in 0..anfield[0].len() - piece_width + 1 {
        if self.position_is_correct(anfield, piece, x, y){
          //todo: implement.
          /*
            check the most agressively placed enemy cell, opposite the major direction
            check the most agressively placed player cell, on the major direction
            update the answer_xy, if the coordinates satisfy at least one of the conditions above(complete the requirements)
          */
        }
      }
    }

    answer_xy
  }

  fn position_is_correct(&self, anfield: &VecDeque<VecDeque<char>>, piece: &VecDeque<VecDeque<char>>, x: usize, y: usize) -> bool {
    //todo: implement.
    /* 
      position is correct if all(except one) non-empty cells of the piece
      are placed on the empty cells of the field, and only one non-empty cell
      of the piece is placed on the player cell(any cell covered by the
      player char by the player peice placement previously)
    */
    false
  }

}