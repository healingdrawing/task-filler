use crate::find_::{Finder, Compas};

impl Finder {
  /** todo check it. the major direction in the first step moment */
  pub fn find_direction(&mut self, from_xy:[usize;2], to_xy:[usize;2])-> Compas {
    let x1 = from_xy[0] as f64;
    let y1 = from_xy[1] as f64;
    let x2 = to_xy[0] as f64;
    let y2 = to_xy[1] as f64;
    let dx = x2-x1;
    let dy = y2-y1;
    let mut angle = dy.atan2(dx);
    if angle < 0.0 {angle += 2.0*std::f64::consts::PI;}
    let angle = angle.to_degrees();
    // println!("angle: {}", angle);
    let direction =
    if angle >= 337.5 || angle < 22.5 { Compas::E } 
    else if angle >= 22.5 && angle < 67.5 { Compas::NE } 
    else if angle >= 67.5 && angle < 112.5 { Compas::N } 
    else if angle >= 112.5 && angle < 157.5 { Compas::NW } 
    else if angle >= 157.5 && angle < 202.5 { Compas::W } 
    else if angle >= 202.5 && angle < 247.5 { Compas::SW } 
    else if angle >= 247.5 && angle < 292.5 { Compas::S } 
    else { Compas::SE };
    direction
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