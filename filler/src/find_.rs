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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MajorStrategy {
  SPEAR, // the first stage, when try to prevent enemy from moving to player side
  FORK, // the second stage, when try to cut the enemy way to half sides angle 45 +-
  CROSS, // the third stage, when try to cut the enemy way to sides angle 90 +-
}

#[derive(Debug)]
pub struct Finder {
  /**first step used to determine major direction*/
  pub fresh: bool,
  /**major strategy */
  pub major_strategy: MajorStrategy,
  /**
   * global minimal distance from most agressive player and enemy cells
   * when it stops decreasing then it is time to change the strategy from SPEAR to FORK
   * */
  pub global_min_distance_between_most_agressive_cells: f64,
  /**major direction to enemy in the beginning*/
  pub major: Compas,
  /**major relative half left direction for fork strategy, let is say 45 degrees+- from major to left */
  pub major_fork_left: Compas,
  /**major relative half right direction for fork strategy, let is say 45 degrees+- from major to right */
  pub major_fork_right: Compas,
  /**major relative left direction */
  pub major_left: Compas,
  /**major relative right direction */
  pub major_right: Compas,
  /**minor direction, opposite to major */
  pub minor: Compas,
  /**for move answer*/
  pub answer_xy: [usize; 2],
  /**for potential implementation of more compact placement(not agressive) */
  pub player_xy: [usize; 2],
  /**for first step*/
  pub first_answer: bool,
}

impl Finder {
  pub fn new() -> Finder {
    Finder {
      fresh: true,
      major_strategy: MajorStrategy::SPEAR,
      global_min_distance_between_most_agressive_cells: f64::MAX,
      major: Compas::CENTRAL,
      major_fork_left: Compas::CENTRAL,
      major_fork_right: Compas::CENTRAL,
      major_left: Compas::CENTRAL,
      major_right: Compas::CENTRAL,
      minor: Compas::CENTRAL,
      answer_xy: [usize::MAX, usize::MAX],
      player_xy: [usize::MAX, usize::MAX],
      first_answer: true,
    }
  }
  
  /** get the answer(coordinates) as String */
  pub fn answer(&mut self) -> String {
    if self.first_answer{
      self.first_answer = false;
      format!("{} {}\n \n + shotgun groin shot", self.answer_xy[0], self.answer_xy[1])
    }
    else{
      format!("{} {}\n \n + shotgun groin shot", self.answer_xy[0], self.answer_xy[1])
    }
  }
  
  pub fn find_answer(&mut self, parser: &mut Parser) {
    if self.fresh{
      self.fresh = false;
      // find the player position
      let player_xy = self.find_player(parser);
      self.player_xy = player_xy;
      // if player not found then surrender
      if player_xy == [usize::MAX, usize::MAX] { self.answer_xy = player_xy; return; }
      // find the enemy position(the most far enemy cell) and save coordinates as surrender answer
      let enemy_xy = self.find_enemy(parser, player_xy);
      // todo: find the enemy direction N(-y) S(+y) W(-x) E(+x), x8 directions using enum,
      self.major = self.find_direction(player_xy, enemy_xy);
      self.major_fork_left = self.find_fork_left_direction(self.major);
      self.major_fork_right = self.find_fork_right_direction(self.major);
      self.major_left = self.find_left_direction(self.major);
      self.major_right = self.find_right_direction(self.major);
      
      self.minor = self.find_opposite_direction(self.major);
    }
    
    /*
    above [only the first step]
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
