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

    [0,0] //todo: remove this gap
  }

}