use crate::{find_::{Finder, Compas, ForkDirection}, debug::{DEBUG_FILE, append_to_file}};

impl Finder {
  
  /**returns x8 directions like compas directions */
  pub fn find_direction(&mut self, from_xy: [usize; 2], to_xy: [usize; 2]) -> Compas {
    let x1 = from_xy[0] as f64;
    let y1 = from_xy[1] as f64;
    let x2 = to_xy[0] as f64;
    let y2 = to_xy[1] as f64;

    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut angle = dy.atan2(dx);

    if angle < 0.0 { angle += 2.0 * std::f64::consts::PI; }

    let angle_degrees = angle.to_degrees();

    let direction =
    if angle_degrees >= 337.5 || angle_degrees < 22.5 { Compas::E }
    else if angle_degrees < 67.5 { Compas::SE }
    else if angle_degrees < 112.5 { Compas::S }
    else if angle_degrees < 157.5 { Compas::SW }
    else if angle_degrees < 202.5 { Compas::W }
    else if angle_degrees < 247.5 { Compas::NW }
    else if angle_degrees < 292.5 { Compas::N }
    else { Compas::NE };

    direction
}

  /** major is longest distance to prefer to be managed first */
  pub fn find_major_and_minor_fork_direction_from_xy(
    &mut self,
    fork_left_direction:Compas,
    fork_right_direction:Compas,
    player_xy:[usize;2],
    anfield_size_xy:&[usize;2],
  )-> [ForkDirection;2] {
    
    let most_far_left_fork_xy = self.find_most_far_xy_of_direction(
      anfield_size_xy,
      fork_left_direction,
    );

    let most_far_right_fork_xy = self.find_most_far_xy_of_direction(
      anfield_size_xy,
      fork_right_direction,
    );

    let left_fork_distance = self.find_distance(
      player_xy,
      most_far_left_fork_xy,
    );

    let right_fork_distance = self.find_distance(
      player_xy,
      most_far_right_fork_xy,
    );

    append_to_file(DEBUG_FILE, &format!("most far right fork xy {:?}", most_far_right_fork_xy)).expect("fail to write to file");
    append_to_file(DEBUG_FILE, &format!("most far left fork xy {:?}", most_far_left_fork_xy)).expect("fail to write to file");
    append_to_file(DEBUG_FILE, &format!("right fork distance{:?}", right_fork_distance)).expect("fail to write to file");
    append_to_file(DEBUG_FILE, &format!("left fork distance{:?}", left_fork_distance)).expect("fail to write to file");

    if right_fork_distance > left_fork_distance {
      [ForkDirection::RIGHT, ForkDirection::LEFT]
    } else {
      [ForkDirection::LEFT, ForkDirection::RIGHT]
    }

  }

  /** find left direction close to 45 degrees angle relative to incoming */
  pub fn find_fork_left_direction(&mut self, incoming:Compas)-> Compas {
    match incoming {
      Compas::N => Compas::NW,
      Compas::S => Compas::SE,
      Compas::W => Compas::SW,
      Compas::E => Compas::NE,
      Compas::NW => Compas::W,
      Compas::NE => Compas::N,
      Compas::SW => Compas::S,
      Compas::SE => Compas::E,
      Compas::CENTRAL => Compas::CENTRAL,
    }
  }

  /** find right direction close to -45 degrees angle relative to incoming */
  pub fn find_fork_right_direction(&mut self, incoming:Compas)-> Compas {
    /*turn full step right to make half step left and get fork right */
    let right = self.find_right_direction(incoming);
    self.find_fork_left_direction( right )
  }

  /** find left direction close to 90 degrees angle relative to incoming */
  pub fn find_left_direction(&mut self, incoming:Compas)-> Compas {
    match incoming {
      Compas::N => Compas::W,
      Compas::S => Compas::E,
      Compas::W => Compas::S,
      Compas::E => Compas::N,
      Compas::NW => Compas::SW,
      Compas::NE => Compas::NW,
      Compas::SW => Compas::SE,
      Compas::SE => Compas::NE,
      Compas::CENTRAL => Compas::CENTRAL,
    }
  }

  /** find right direction close to -90 degrees relative to incoming */
  pub fn find_right_direction(&mut self, incoming:Compas)-> Compas {
    let opposite = self.find_opposite_direction(incoming);
    self.find_left_direction(opposite)
  }

  /** find direction opposite to incoming */
  pub fn find_opposite_direction(&mut self, incoming:Compas)-> Compas {
    match incoming {
      Compas::N => Compas::S,
      Compas::S => Compas::N,
      Compas::W => Compas::E,
      Compas::E => Compas::W,
      Compas::NW => Compas::SE,
      Compas::NE => Compas::SW,
      Compas::SW => Compas::NE,
      Compas::SE => Compas::NW,
      Compas::CENTRAL => Compas::CENTRAL,
    }
  }
  
}