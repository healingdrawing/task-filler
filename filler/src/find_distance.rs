use std::collections::VecDeque;

use crate::find_::{Finder, Compas};

impl Finder {
  pub fn find_distance(&mut self, xy1:[usize;2], xy2:[usize;2])-> f64 {
    let x1 = xy1[0] as f64;
    let y1 = xy1[1] as f64;
    let x2 = xy2[0] as f64;
    let y2 = xy2[1] as f64;
    ((x1-x2).powf(2.0) + (y1-y2).powf(2.0)).sqrt()
  }
  
  /** find the minimum distance from the any cell of the team
  * to the cell of the opposite_team_cell_xy,
  * according to symbol of anfield cell.
  * if opposite_team_cell_xy of anfield is a or @,
  * then the team cells is s or $,
  */
  pub fn find_min_distance_from_team_area_to_opposite_team_cell(&mut self,anfield: &VecDeque<VecDeque<char>>, opposite_team_cell_xy:[usize;2])-> f64 {
    let team_char =
    if anfield[opposite_team_cell_xy[1]][opposite_team_cell_xy[0]] == 'a'
    || anfield[opposite_team_cell_xy[1]][opposite_team_cell_xy[0]] == '@'
    {
      ['s','$']
    }
    else if anfield[opposite_team_cell_xy[1]][opposite_team_cell_xy[0]] == 's'
    || anfield[opposite_team_cell_xy[1]][opposite_team_cell_xy[0]] == '$'
    {
      ['a','@']
    }
    else {
      ['\0','\0']
    };
    
    /*
    if incoming char coordinates were wrong, then return min value
    it will prevent to have result shorter than this distance,
    so the variant will be not chosen as effective
    */
    if team_char[0] == '\0' || team_char[1] == '\0' { return f64::MIN; }
    
    let mut distance = f64::MAX;
    for y in 0..anfield.len() {
      for x in 0..anfield[0].len() {
        if anfield[y][x] == team_char[0] || anfield[y][x] == team_char[1] {
          let xy = [x,y];
          let current_distance = self.find_distance(xy, opposite_team_cell_xy);
          if current_distance < distance {
            distance = current_distance;
          }
        }
      }
    }
    distance
    
  }
  
  /**find the most agressive distance of piece cell on compas way */
  pub fn find_most_agressive_distance_of_piece_cell(
    &mut self,
    piece: &VecDeque<VecDeque<char>>,
    piece_left_top_cell_xy:[i128;2],
    direction:Compas,
    anfield_size:&[usize;2],
  )-> f64 {
    let mut distance = f64::MIN;
    let xy = [piece_left_top_cell_xy[0], piece_left_top_cell_xy[1]];
    let far_xy = self.find_most_far_xy_opposite_the_direction(
      anfield_size,
      direction
    );
    
    /* iterate each piece cell */
    for (piece_y, piece_row) in piece.iter().enumerate() {
      for (piece_x, piece_cell) in piece_row.iter().enumerate() {
        /* if the cell is not empty */
        if *piece_cell != '.' {
          let x = piece_x as i128 +xy[0];
          let y = piece_y as i128 +xy[1];
          /*from far point to piece cell */
          let piece_distance = self.find_distance(
            far_xy,
            [x as usize, y as usize]
          );
          
          if piece_distance > distance {
            distance = piece_distance;
          }
        }
      }
    }
    
    distance
  }
  
  /**find the most agressive distance of piece cell on on compas way */
  pub fn find_most_agressive_distance_proportion_of_piece_cell(
    &mut self,
    piece: &VecDeque<VecDeque<char>>,
    piece_left_top_cell_xy:[i128;2],
    direction:Compas,
    anfield_size:&[usize;2],
  )-> f64 {
    
    let full_distance = self.find_full_distance_of_direction(
      anfield_size,
      direction
    );
    
    if full_distance == 0f64 {
      println!("wtf? looks like anfield_size is zero or direction is central");
      return 0f64;
    }
    
    let mut distance = 0f64;
    let xy = piece_left_top_cell_xy.clone();
    let far_xy = self.find_most_far_xy_opposite_the_direction(
      anfield_size,
      direction
    );
    
    /* iterate each piece cell */
    for (piece_y, piece_row) in piece.iter().enumerate() {
      for (piece_x, piece_cell) in piece_row.iter().enumerate() {
        /* if the cell is not empty */
        if *piece_cell != '.' {
          let x = piece_x as i128 + xy[0];
          let y = piece_y as i128 + xy[1];
          /*from far point to piece cell */
          let piece_distance = self.find_distance(
            far_xy.clone(),
            [x as usize, y as usize].clone()
          );
          
          if piece_distance > distance {
            distance = piece_distance.clone();
          }
        }
      }
    }
    
    distance / full_distance
  }
  
  /**find the most agressive distance of piece cell on on compas way */
  pub fn find_less_agressive_distance_proportion_of_piece_cell(
    &mut self,
    piece: &VecDeque<VecDeque<char>>,
    piece_left_top_cell_xy:[i128;2],
    direction:Compas,
    anfield_size:&[usize;2],
  )-> f64 {
    
    let full_distance = self.find_full_distance_of_direction(
      anfield_size,
      direction
    );
    
    if full_distance == 0f64 {
      println!("wtf? looks like anfield_size is zero or direction is central");
      return 0f64;
    }
    
    let mut distance = 0f64;
    let xy = piece_left_top_cell_xy.clone();
    let far_xy = self.find_most_far_xy_of_direction(
      anfield_size,
      direction
    );
    
    /* iterate each piece cell */
    for (piece_y, piece_row) in piece.iter().enumerate() {
      for (piece_x, piece_cell) in piece_row.iter().enumerate() {
        /* if the cell is not empty */
        if *piece_cell != '.' {
          let x = piece_x as i128 + xy[0];
          let y = piece_y as i128 + xy[1];
          /*from far point to piece cell */
          let piece_distance = self.find_distance(
            far_xy.clone(),
            [x as usize, y as usize].clone()
          );
          
          if piece_distance < distance {
            distance = piece_distance.clone();
          }
        }
      }
    }
    
    distance / full_distance
  }
  
  
  fn find_full_distance_of_direction(
    &mut self,
    anfield_size:&[usize;2],
    direction:Compas
  )-> f64 {
    match direction {
      Compas::N | Compas::S => anfield_size[1] as f64,
      Compas::W | Compas::E => anfield_size[0] as f64,
      Compas::NW | Compas::SE | Compas::NE | Compas::SW => (
        (
          anfield_size[0] * anfield_size[0]
          + anfield_size[1] * anfield_size[1]
        ) as f64
      ).sqrt(),
      Compas::CENTRAL => 0f64,
    }
  }
  
  // File: forward_check.rs
  
  fn find_distance_to_vector(&mut self, vector_start_xy: [f64;2], vector_end_xy: [f64;2], xy: [f64;2]) -> f64 {
    let x0 = vector_start_xy[0];
    let y0 = vector_start_xy[1];
    let x1 = vector_end_xy[0];
    let y1 = vector_end_xy[1];
    let x2 = xy[0];
    let y2 = xy[1];
    
    // Calculate the vector components
    let vector_x = x1 - x0;
    let vector_y = y1 - y0;
    
    // Calculate the distance from p2 to the vector using the cross product
    let cross_product = (x2 - x0) * vector_y - (y2 - y0) * vector_x;
    let distance = cross_product.abs() / (vector_x.powi(2) + vector_y.powi(2)).sqrt();
    
    distance
  }
  
  fn is_point_forward_of_vector(&mut self, vector_start_xy: [f64;2], vector_end_xy: [f64;2], xy: [f64;2]) -> bool {
    let x0 = vector_start_xy[0];
    let y0 = vector_start_xy[1];
    let x1 = vector_end_xy[0];
    let y1 = vector_end_xy[1];
    let x2 = xy[0];
    let y2 = xy[1];

    // Calculate the vector components
    let vector_x = x1 - x0;
    let vector_y = y1 - y0;
    
    // Calculate the dot product
    let dot_product = (x2 - x0) * vector_x + (y2 - y0) * vector_y;
    
    // Calculate the scalar projection
    let scalar_projection = dot_product / (vector_x.powi(2) + vector_y.powi(2));
    
    // Check if the projection is between p0 and p1, or more forward than p1
    scalar_projection >= 0.0 // && scalar_projection <= 1.0
  }
  
  pub fn find_min_distance_from_piece_to_vector(
    &mut self,
    piece: &VecDeque<VecDeque<char>>,
    piece_left_top_cell_xy:[i128;2],
    vector_start_point: [i128;2],
    vector_end_point: [i128;2],
  )-> f64 {
    let mut distance = f64::MAX;
    let xy = [piece_left_top_cell_xy[0], piece_left_top_cell_xy[1]];

    /* iterate each piece cell */
    for (piece_y, piece_row) in piece.iter().enumerate() {
      for (piece_x, piece_cell) in piece_row.iter().enumerate() {
        /* if the cell is not empty */
        if *piece_cell != '.' {
          let x = piece_x as i128 +xy[0];
          let y = piece_y as i128 +xy[1];
          /*from far point to piece cell */
          let piece_distance = self.find_distance_to_vector(
            [vector_start_point[0] as f64, vector_start_point[1] as f64],
            [vector_end_point[0] as f64, vector_end_point[1] as f64],
            [x as f64, y as f64]
          );

          if piece_distance < distance {
            distance = piece_distance;
          }
        }
      }
    }
    distance
  }
  
  
}