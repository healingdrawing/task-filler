use crate::parse_::Parser;

/** direction of the vector aimed to enemy */
#[derive(Debug)]
pub enum Compas {
  N, // -y 
  NE, // +x -y
  E, // +x
  SE, // +x +y
  S, // +y
  SW, // -x +y
  W, // -x
  NW, // -x -y
  CENTRAL, // no direction
}

#[derive(Debug)]
pub struct Finder {
  /**first step used to determine major direction*/
  pub fresh: bool,
  /**major direction */
  pub major: Compas,
  /**for move answer*/
  pub answer_xy: [usize; 2],
  /**for surrender answer*/
  pub enemy_xy: [usize; 2],
  // pub piece: Vec<Vec<char>>,
  // pub field: Vec<Vec<char>>,
  // pub variants: Vec<[usize; 2]>,
}

impl Finder {
  pub fn new() -> Finder {
    Finder {
      fresh: true,
      major: Compas::CENTRAL,
      answer_xy: [0, 0],
      enemy_xy: [0, 0],
    }
  }

  /** get the answer(coordinates) as String */
  pub fn answer(&self) -> String {format!("{} {}", self.answer_xy[0], self.answer_xy[1])}
  
  pub fn find_answer(&mut self, parser: &mut Parser) {
    let anfield = &parser.anfield;
    let piece = &parser.piece;
    let player = &parser.player_char;
    
    // todo: implement find solution here
    //first clean the raw data of the field, find solution and write it to self.answer
    
    // clean the data
    // build the 2d array field (row,column)yx = Vec<Vec<char>> , to represent the field

    if self.fresh{
      self.fresh = false;
      // find the player position
      let player_xy = self.find_player(parser);
      // if player not found then surrender
      if player_xy == [usize::MAX, usize::MAX] { self.answer_xy = player_xy; return; }
      // find the enemy position(the most far enemy cell) and save coordinates as surrender answer
      let enemy = self.find_enemy(parser, player_xy);
      self.enemy_xy = enemy;
      // todo: find the enemy direction N(-y) S(+y) W(-x) E(+x), x8 directions using enum,
      
    }

    /*
    [only the first step]
    find the player position
    find the enemy position(the most far enemy cell) and save coordinates as surrender answer
    find the enemy direction N(-y) S(+y) W(-x) E(+x), x8 directions using enum,
    that is the new piece major direction and save it into finder.
    Later it can be used as way
    to find the enemy cells which are directed to the player size
    and moved already as possible deep in player direction,
    to cut their way first if it is possible.
     */

    // find the most agressively placed enemy cell, opposite the major direction

    // find the most agressively placed player cell, on the major direction

    // find all possible variants to place the piece on the field correctly

    // iterate the correct variants
    // check if there is variant to place the piece on the field that some cell of that piece
    // will be placed not the less agressively than the enemy cell, so the same or more agressively
    // from the enemy position. if there is such variant - place the piece there(set the self.answer)
    // to try prevent or restrict the enemy cell to move in the major direction

    // otherwise iterate the correct variants again
    // and place the piece on the field with the most agressively placed player cell, as possible deep
    // in the major direction, to try cover more enemy cells in the major direction

    // otherwise return the surrender answer (the most far enemy position)


    self.answer_xy = [2, 2]; //todo: remove this line, it is only for test
    
    //clean parser
    parser.reset();
    
  }
  
}
