use std::collections::VecDeque;

use crate::find_::{Finder, Compas};



impl Finder {
  pub fn find_more_agressive(
    &mut self,
    piece: &VecDeque<VecDeque<char>>,
    piece_top_left_xy: &[usize; 2],
    most_agressive_xy: &[usize; 2],
    player_major_direction: &Compas,
    anfield_size_xy: &[usize; 2],
  )-> [usize; 2] {
    let xy = piece_top_left_xy;
    let asxy = anfield_size_xy;
    /*
      determine the most far point of anfield, according to player_major_direction
    */
    // let far_xy = 
    // match player_major_direction {
    //   Compas::N => {[(asxy[0] - 1)/2, asxy[1] - 1]},
    //   Compas::S => {[(asxy[0] - 1)/2, 0]},
    //   Compas::W => {[asxy[0] - 1, (asxy[1] - 1)/2]},
    //   Compas::E => {[0, (asxy[1] - 1)/2]},
    //   Compas::NW => {[asxy[0] - 1, asxy[1] - 1]},
    //   Compas::NE => {[0, asxy[1] - 1]},
    //   Compas::SW => {[asxy[0] - 1, 0]},
    //   Compas::SE => {[0, 0]},
    //   Compas::CENTRAL => {[usize::MAX, usize::MAX]}, /* should never fire */
    // };

    let far_xy = self.find_most_far_xy_opposite_the_direction(
      asxy,
      player_major_direction.clone()
    );

    let mut most_agressive_xy = most_agressive_xy.clone();

    /* iterate each piece cell */
    for (piece_y, piece_row) in piece.iter().enumerate() {
      for (piece_x, piece_cell) in piece_row.iter().enumerate() {
        /* if the cell is not empty */
        if *piece_cell != '.' {
          let x = piece_x+xy[0];
          let y = piece_y+xy[1];
          /*from far point to piece cell */
          let piece_distance = self.find_distance(
            far_xy,
            [x,y]
          );
          /* the last calculated most distance */
          let most_distance = self.find_distance(
            far_xy,
            most_agressive_xy
          );

          let piece_distance_longer = piece_distance > most_distance;

          match player_major_direction {
            Compas::N => {
              if y < most_agressive_xy[1]
              || y == most_agressive_xy[1] && piece_distance_longer {
                most_agressive_xy = [x,y];
              }
            },
            Compas::S => {
              if y > most_agressive_xy[1]
              || y == most_agressive_xy[1] && piece_distance_longer {
                most_agressive_xy = [x,y];
              }
            },
            Compas::W => {
              if x < most_agressive_xy[0]
              || x == most_agressive_xy[0] && piece_distance_longer {
                most_agressive_xy = [x,y];
              }
            },
            Compas::E => {
              if x > most_agressive_xy[0]
              || x == most_agressive_xy[0] && piece_distance_longer {
                most_agressive_xy = [x,y];
              }
            },
            Compas::NW => {
              if x < most_agressive_xy[0] && y < most_agressive_xy[1]
              || (
                x == most_agressive_xy[0] && y < most_agressive_xy[1]
                || x < most_agressive_xy[0] && y == most_agressive_xy[1]
              ) && piece_distance_longer {
                most_agressive_xy = [x,y];
              }
            },
            Compas::NE => {
              if x > most_agressive_xy[0] && y < most_agressive_xy[1]
              || (
                x == most_agressive_xy[0]  && y < most_agressive_xy[1]
                || x > most_agressive_xy[0] && y == most_agressive_xy[1]  
              ) && piece_distance_longer {
                most_agressive_xy = [x,y];
              }
            },
            Compas::SW => {
              if x < most_agressive_xy[0] && y > most_agressive_xy[1]
              || (
                x == most_agressive_xy[0] && y > most_agressive_xy[1]
                || x < most_agressive_xy[0] && y == most_agressive_xy[1]
              ) && piece_distance_longer {
                most_agressive_xy = [x,y];
              }
            },
            Compas::SE => {
              if x > most_agressive_xy[0] && y > most_agressive_xy[1]
              || (
                x == most_agressive_xy[0] && y > most_agressive_xy[1]
                || x > most_agressive_xy[0] && y == most_agressive_xy[1]
              ) && piece_distance_longer {
                most_agressive_xy = [x,y];
              }
            },
            Compas::CENTRAL => {}, /* should never fire */
          }
          
        }
      }
    }

    most_agressive_xy
  }

  // pub fn first_more_agressive_than_second(
  //   &mut self,
  //   first_xy: &[usize; 2],
  //   second_xy: &[usize; 2],
  //   player_major_direction: &Compas,
  //   anfield_size_xy: &[usize; 2],
  // ) -> bool {
  //   let asxy = anfield_size_xy;

  //   /*
  //     determine the most far point of anfield, according to player_major_direction
  //   */
  //   let far_xy = 
  //   match player_major_direction {
  //     Compas::N => {[(asxy[0] - 1)/2, asxy[1] - 1]},
  //     Compas::S => {[(asxy[0] - 1)/2, 0]},
  //     Compas::W => {[asxy[0] - 1, (asxy[1] - 1)/2]},
  //     Compas::E => {[0, (asxy[1] - 1)/2]},
  //     Compas::NW => {[asxy[0] - 1, asxy[1] - 1]},
  //     Compas::NE => {[0, asxy[1] - 1]},
  //     Compas::SW => {[asxy[0] - 1, 0]},
  //     Compas::SE => {[0, 0]},
  //     Compas::CENTRAL => {[usize::MAX, usize::MAX]}, /* should never fire */
  //   };

  //   /*from far point to first cell */
  //   let first_distance = self.find_distance(
  //     far_xy,
  //     *first_xy
  //   );

  //   /*from far point to second cell */
  //   let second_distance = self.find_distance(
  //     far_xy,
  //     *second_xy
  //   );

  //   let first_distance_longer = first_distance > second_distance;

  //   match player_major_direction {
  //     Compas::N => {
  //       first_xy[1] < second_xy[1]
  //       || first_xy[1] == second_xy[1] && first_distance_longer 
  //     },
  //     Compas::S => {
  //       first_xy[1] > second_xy[1]
  //       || first_xy[1] == second_xy[1] && first_distance_longer
  //     },
  //     Compas::W => {
  //       first_xy[0] < second_xy[0]
  //       || first_xy[0] == second_xy[0] && first_distance_longer
  //     },
  //     Compas::E => {
  //       first_xy[0] > second_xy[0]
  //       || first_xy[0] == second_xy[0] && first_distance_longer
  //     },
  //     Compas::NW => {
  //       first_xy[0] < second_xy[0] && first_xy[1] < second_xy[1]
  //       || (
  //         first_xy[0] == second_xy[0] && first_xy[1] < second_xy[1]
  //         || first_xy[0] < second_xy[0] && first_xy[1] == second_xy[1]
  //       ) && first_distance_longer
  //     },
  //     Compas::NE => {
  //       first_xy[0] > second_xy[0] && first_xy[1] < second_xy[1]
  //       || (
  //         first_xy[0] == second_xy[0] && first_xy[1] < second_xy[1]
  //         || first_xy[0] > second_xy[0] && first_xy[1] == second_xy[1]
  //       ) && first_distance_longer
  //     },
  //     Compas::SW => {
  //       first_xy[0] < second_xy[0] && first_xy[1] > second_xy[1]
  //       || (
  //         first_xy[0] == second_xy[0] && first_xy[1] > second_xy[1]
  //         || first_xy[0] < second_xy[0] && first_xy[1] == second_xy[1]
  //       ) && first_distance_longer
  //     },
  //     Compas::SE => {
  //       first_xy[0] > second_xy[0] && first_xy[1] > second_xy[1]
  //       || (
  //         first_xy[0] == second_xy[0] && first_xy[1] > second_xy[1]
  //         || first_xy[0] > second_xy[0] && first_xy[1] == second_xy[1]
  //       ) && first_distance_longer
  //     },
  //     Compas::CENTRAL => {false}, /* should never fire */
  //   }

  // }
    
}