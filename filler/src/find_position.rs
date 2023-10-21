use std::collections::VecDeque;

use crate::{find_::{Finder, Compas, MajorStrategy, ForkDirection}, parse_::Parser, debug::{DEBUG_FILE, append_to_file}};

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
  pub fn find_position(&mut self, parser: &mut Parser) -> [i128;2] {
    let anfield = &parser.anfield;
    let piece = &parser.piece;
    self.piece_negative_xy = parser.piece_negative_xy.clone();
    
    let player_char = &parser.player_char.clone();
    let enemy_char = &parser.enemy_char.clone(); // used to find middle arithmetical position of enemy field
    
    /* for more agressive piece cell check */
    let anfield_size_xy = [anfield[0].len(), anfield.len()];
    
    /* the piece top left corner position */
    let mut answer_xy = [i128::MIN, i128::MIN];
    
    //todo: implement.
    /* to default values for correct piece position */
    let mut fresh_calculation:bool = true;
    
    let piece_height = piece.len() as i128;
    let piece_width = piece[0].len() as i128;
    let piece_diagonal_proportion =
    self.diagonal_of_the_piece_not_empty_cells_rectangle(piece)/self.diagonal_of_the_piece_not_empty_cells_rectangle(anfield);
    
    // append_to_file(DEBUG_FILE, &format!("\n\n===\ninside find_position piece_height {} piece_width {}", piece_height,piece_width)).expect("cannot write to debug file");
    
    /* prepare iterators depend on self.major direction */
    let y_iterator = match self.major {
      Compas::NE | Compas::N | Compas::NW | Compas::W | Compas::CENTRAL =>
      (0-self.piece_negative_xy[1] as i128 .. anfield.len() as i128
      - (piece_height+ self.piece_negative_xy[1] as i128) + 1 )
      .rev()
      .collect::<Vec<_>>().into_iter(),
      
      Compas::SW | Compas::S | Compas::SE | Compas::E =>
      (0-self.piece_negative_xy[1] as i128 .. anfield.len() as i128
      - (piece_height + self.piece_negative_xy[1] as i128) + 1 )
      .collect::<Vec<_>>().into_iter(),
    };
    
    let x_iterator = match self.major {
      Compas::NW | Compas::W | Compas::SW | Compas::S =>
      (0-self.piece_negative_xy[0] as i128 .. anfield[0].len() as i128
      - (piece_width + self.piece_negative_xy[0] as i128) + 1 )
      .rev()
      .collect::<Vec<_>>().into_iter(),
      
      Compas::SE | Compas::E | Compas::NE | Compas::N | Compas::CENTRAL =>
      (0-self.piece_negative_xy[0] as i128 .. anfield[0].len() as i128
      - (piece_width + self.piece_negative_xy[0] as i128) + 1 )
      .collect::<Vec<_>>().into_iter(),
    };
    
    //todo: SPEAR does not used at the moment, because less effective finally, fail lot of cases on the audit question
    /*
    if self.major_strategy == MajorStrategy::SPEAR {/*agressively invade face direction */
      let mut spear_strategy_still_effective = false; /*false - no more ways to decrease distance from piece to most enemy agressive cell*/
      for y in y_iterator.clone() {
        for x in x_iterator.clone() {
          if self.position_is_correct(anfield, piece, x, y, player_char){
            
            if fresh_calculation {/*use the first found correct position for the piece as default */
              fresh_calculation = false;
              answer_xy = [x, y].clone();
            }
            
            /* the most argessive enemy cell position */
            let most_agressive_enemy_xy = self.find_most_agressive_xy(&parser, enemy_char, self.minor.clone());
            
            let min_distance_from_piece_to_most_agressive_enemy_xy =
            self.find_piece_cell_min_distance_to_cell_xy(
              anfield, piece,
              [x,y].clone(),
              most_agressive_enemy_xy.clone()
            );
            
            if min_distance_from_piece_to_most_agressive_enemy_xy <
            self.global_min_distance_between_most_agressive_cells
            { /*still possible decrease the distance to most agressive enemy cell */
              self.global_min_distance_between_most_agressive_cells = min_distance_from_piece_to_most_agressive_enemy_xy.clone();
              spear_strategy_still_effective = true;
              answer_xy = [x, y].clone();
              
            }
            
          }
        }
      }
      
      if spear_strategy_still_effective {
        return answer_xy;
      } else {
        self.major_strategy = MajorStrategy::FORK;
        fresh_calculation = true;
      }
      
    }
    */
    
    let mut fork_strategy_still_effective = false; /*false - no more ways to increase the progress to fork directions*/
    
    if self.major_strategy == MajorStrategy::FORK {/*agressively invade to fork sides */
      
      let mut major_fork_strategy_still_effective = false;
      let mut minor_fork_strategy_still_effective = false;
      
      /* use buffers to rollback finally not used variant
      * after choose which one to use,
      * left or right fork
      */
      let mut buffer_global_max_distance_proportion_major_fork =
      self.global_max_distance_proportion_major_fork.clone();
      
      let mut buffer_global_max_distance_proportion_minor_fork =
      self.global_max_distance_proportion_minor_fork.clone();
      
      let mut answer_xy_major_fork = answer_xy.clone();
      let mut answer_xy_minor_fork = answer_xy.clone();
      
      for y in y_iterator.clone() {
        for x in x_iterator.clone() {
          if self.position_is_correct(anfield, piece, x, y, player_char){
            
            if fresh_calculation {/*use the first found correct position for the piece as default */
              fresh_calculation = false;
              answer_xy = [x, y].clone();
              answer_xy_major_fork = [x, y].clone();
              answer_xy_minor_fork = [x, y].clone();
            }
            
            let max_distance_proportion_left_fork =
            self.find_most_agressive_distance_proportion_of_piece_cell(
              piece,
              [x,y].clone(),
              self.major_fork_left.clone(),
              &anfield_size_xy.clone()
            );
            
            let max_distance_proportion_right_fork =
            self.find_most_agressive_distance_proportion_of_piece_cell(
              piece,
              [x,y].clone(),
              self.major_fork_right.clone(),
              &anfield_size_xy.clone()
            );
            
            let [max_distance_major_fork, max_distance_minor_fork] = match self.major_fork_direction {
              ForkDirection::LEFT => [max_distance_proportion_left_fork, max_distance_proportion_right_fork],
              ForkDirection::RIGHT => [max_distance_proportion_right_fork, max_distance_proportion_left_fork],
            };
            
            if max_distance_major_fork > buffer_global_max_distance_proportion_major_fork - piece_diagonal_proportion
            {
              buffer_global_max_distance_proportion_major_fork = max_distance_major_fork.clone();
              fork_strategy_still_effective = true;
              major_fork_strategy_still_effective = true;
              answer_xy_major_fork = [x, y].clone();
            }
            
            if max_distance_minor_fork > buffer_global_max_distance_proportion_minor_fork - piece_diagonal_proportion
            {
              buffer_global_max_distance_proportion_minor_fork = max_distance_minor_fork.clone();
              fork_strategy_still_effective = true;
              minor_fork_strategy_still_effective = true;
              answer_xy_minor_fork = [x, y].clone();
            }
            
          }
        }
      }
      
      if fork_strategy_still_effective
      {
        
        if major_fork_strategy_still_effective{
          
          self.global_max_distance_proportion_major_fork = buffer_global_max_distance_proportion_major_fork.clone();
          return answer_xy_major_fork.clone();
          
        } 
        if minor_fork_strategy_still_effective{
          
          self.global_max_distance_proportion_minor_fork = buffer_global_max_distance_proportion_minor_fork.clone();
          return answer_xy_minor_fork.clone();
          
        }
        
        
        
      }
      
    } // end of fork strategy
    
    
    // if strategies did not work, 
    
    // now try to find the peice which is the most closer/far(try both) to the enemy field, according to the middle arithmetical position of the enemy cells, and the middle arithmetical position of the piece cells
    
    // append_to_file(DEBUG_FILE, &format!("\n\n===\nAFTER FORK")).expect("cannot write to debug file");
    if false { /*turn on the after FORK strategy */
      let enemy_cells_middle_xy = self.find_middle_arithmetical_xy_position_of_enemy_field(anfield, enemy_char);
      
      let mut first_step = true; // manage the first position of the piece, to use it as default answer
      let mut distance = 0f64; // manage the distance from the piece to the enemy field
      
      for y in y_iterator.clone() {
        for x in x_iterator.clone() {
          if self.position_is_correct(anfield, piece, x, y, player_char){
            
            if fresh_calculation {/*use the first found correct position for the piece as default */
              fresh_calculation = false;
              answer_xy = [x, y].clone();
            }
            
            let piece_cells_middle_xy = self.find_middle_arithmetical_xy_position_of_piece(
              piece,
              [x,y].clone()
            );
            
            let distance_from_piece_to_enemy_cells_middle_xy =
            self.find_distance(
              piece_cells_middle_xy.clone(),
              enemy_cells_middle_xy.clone()
            );
            
            if first_step {
              first_step = false;
              distance = distance_from_piece_to_enemy_cells_middle_xy.clone();
              answer_xy = [x, y].clone();
            } else if distance_from_piece_to_enemy_cells_middle_xy < distance {
              distance = distance_from_piece_to_enemy_cells_middle_xy.clone();
              answer_xy = [x, y].clone();
            }
            
            
          }
        }
      }
      
    }
    
    append_to_file(DEBUG_FILE, &format!("\n====\nanswer_xy: {} {}", &answer_xy[0], &answer_xy[1])).expect("cannot write to debug file");
    answer_xy
  }
  
  /** the piece position is correct if all(except one) non-empty cells of the piece are placed on the empty cells of the field, and only one non-empty cell of the piece is placed on the player cell(any cell covered by the player char by the player piece placement previously) */
  fn position_is_correct(
    &mut self,
    anfield: &VecDeque<VecDeque<char>>,
    piece: &VecDeque<VecDeque<char>>,
    x: i128, y: i128, player:&[char;2]
  ) -> bool {
    
    // append_to_file(DEBUG_FILE, &format!("\n\n===\ninside position is correct")).expect("cannot write to debug file");
    // // append_to_file(DEBUG_FILE, &format!("anfield {:?}" ,anfield)).expect("cannot write to debug file");
    // append_to_file(DEBUG_FILE, &format!("piece {:?}" ,piece)).expect("cannot write to debug file");
    // append_to_file(DEBUG_FILE, &format!("x {} y {}" ,x,y)).expect("cannot write to debug file");
    // append_to_file(DEBUG_FILE, &format!("neg_x {} neg_y {}" ,self.piece_negative_xy[0],self.piece_negative_xy[1])).expect("cannot write to debug file");
    
    
    // append_to_file(DEBUG_FILE, &format!("player {:?}" ,player)).expect("cannot write to debug file");
    
    /*
    only one cell from the piece must be placed on the player cell, so
    when the player_cells_hovered_by_piece is 1, for all the piece cells,
    the position is correct, otherwise it is not
    */
    let mut player_cells_hovered_by_piece:i128 = 0;
    
    /*manage negative indices */
    let zip_y = (y + self.piece_negative_xy[1] as i128) as usize;
    let zip_x = (x + self.piece_negative_xy[0] as i128) as usize;
    
    // append_to_file(DEBUG_FILE, &format!("zip_x {} zip_y {}" ,zip_x,zip_y)).expect("cannot write to debug file");
    
    /*iterate the piece and compare the cells with the field cells using the x and y incrementation*/
    for (piece_y, field_y) in (0..piece.len()).zip(zip_y..zip_y + piece.len()) { /*vertical row step */
      for (piece_x, field_x) in (0..piece[0].len()).zip(zip_x..zip_x+piece[0].len()) {/*column */
        if piece[piece_y][piece_x] != '.' {/*if the piece cell is not empty*/
          if anfield[field_y][field_x] != '.' {/*if the field cell is not empty*/
            /* both cells (anfield, piece) are not empty, so need extra check*/
            
            if player_cells_hovered_by_piece > 0{/*if at least one player cell is already hovered by piece*/
              return false;/*the piece position is not correct*/
            }
            
            if anfield[field_y][field_x] == player[0] || anfield[field_y][field_x] == player[1] {/*if the field cell is player cell*/
              player_cells_hovered_by_piece += 1;/*increment the player cells hovered by piece*/
            } else {/*if the field cell is enemy cell*/
              return false;/*the piece position is not correct*/
            }
            
          }
        }
      }
    }
    
    if player_cells_hovered_by_piece == 0 {/*if(finally) no player cell is hovered by piece*/
      return false;/*the piece position is not correct*/
    }
    // append_to_file(DEBUG_FILE, "the piece position is correct!!!\n\n").expect("cannot write to debug file");
    true /*the piece position is correct*/
  }
  
  fn find_middle_arithmetical_xy_position_of_enemy_field(
    &mut self,
    anfield: &VecDeque<VecDeque<char>>,
    enemy_char: &[char;2],
  ) -> [usize;2] {
    let mut enemy_cells_count = 0;
    let mut enemy_cells_x_sum = 0;
    let mut enemy_cells_y_sum = 0;
    
    for (y, row) in anfield.iter().enumerate() {
      for (x, cell) in row.iter().enumerate() {
        if cell == &enemy_char[0] || cell == &enemy_char[1] {
          enemy_cells_count += 1;
          enemy_cells_x_sum += x;
          enemy_cells_y_sum += y;
        }
      }
    }
    
    let enemy_cells_x_middle = enemy_cells_x_sum as f64 / enemy_cells_count as f64;
    let enemy_cells_y_middle = enemy_cells_y_sum as f64 / enemy_cells_count as f64;
    
    [enemy_cells_x_middle as usize, enemy_cells_y_middle as usize]
    
  }
  
  fn find_middle_arithmetical_xy_position_of_piece(
    &mut self,
    piece: &VecDeque<VecDeque<char>>,
    top_left_piece_corner_xy: [i128;2], // indices on anfield where piece placed, potentially negative too
  ) -> [usize;2] {
    let mut piece_cells_count = 0;
    let mut piece_cells_x_sum = 0;
    let mut piece_cells_y_sum = 0;
    
    for (y, row) in piece.iter().enumerate() {
      for (x, cell) in row.iter().enumerate() {
        if cell != &'.' {
          piece_cells_count += 1;
          piece_cells_x_sum += x as i128 + top_left_piece_corner_xy[0];
          piece_cells_y_sum += y as i128 + top_left_piece_corner_xy[1];
        }
      }
    }
    
    let piece_cells_x_middle = piece_cells_x_sum as f64 / piece_cells_count as f64;
    let piece_cells_y_middle = piece_cells_y_sum as f64 / piece_cells_count as f64;
    
    [piece_cells_x_middle as usize, piece_cells_y_middle as usize]
    
  }
  
  pub fn reset_negative_xy(&mut self){
    self.piece_negative_xy = [usize::MIN, usize::MIN];
  }
  
}