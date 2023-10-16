use crate::parse_::Parser;

/** direction of the vector aimed to enemy */
#[derive(Debug, Clone, Copy)]
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
  /**major direction to enemy in the beginning*/
  pub major: Compas,
  /**major relative left direction */
  pub major_left: Compas,
  /**major relative right direction */
  pub major_right: Compas,
  /**minor direction, opposite to major */
  pub minor: Compas,
  /**for move answer*/
  pub answer_xy: [usize; 2],
  /**for surrender answer*/
  pub enemy_xy: [usize; 2],
  /**for potential implementation of more compact placement(not agressive) */
  pub player_xy: [usize; 2],
}

impl Finder {
  pub fn new() -> Finder {
    Finder {
      fresh: true,
      major: Compas::CENTRAL,
      major_left: Compas::CENTRAL,
      major_right: Compas::CENTRAL,
      minor: Compas::CENTRAL,
      answer_xy: [usize::MAX, usize::MAX],
      enemy_xy: [usize::MAX, usize::MAX],
      player_xy: [usize::MAX, usize::MAX],
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
      self.player_xy = player_xy;
      // if player not found then surrender
      if player_xy == [usize::MAX, usize::MAX] { self.answer_xy = player_xy; return; }
      // find the enemy position(the most far enemy cell) and save coordinates as surrender answer
      let enemy_xy = self.find_enemy(parser, player_xy);
      self.enemy_xy = enemy_xy;
      // todo: find the enemy direction N(-y) S(+y) W(-x) E(+x), x8 directions using enum,
      self.major = self.find_direction(player_xy, enemy_xy);
      self.major_left = self.find_left_direction(self.major);
      self.major_right = self.find_right_direction(self.major);
      
      self.minor = self.find_opposite_direction(self.major);
    }

    /*
    [only the first step]
    find the player position
    find the enemy position(the most far enemy cell) and save coordinates as surrender answer
    find the enemy direction N(-y) S(+y) W(-x) E(+x), x8 directions using enum,
    that is the new piece major direction and save it into finder.
    Later it can be used as way
    to find the enemy cells which are directed to the player side
    and moved already as possible deep in player direction,
    to cut their way first if it is possible.
     */

    self.answer_xy = self.find_position(parser); //todo: implement. it is raw
    
    //clean parser
    parser.reset();
    
  }
  
}
