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
    let enemy_char = &parser.enemy_char.clone();
    
    /* for more agressive piece cell check */
    let anfield_size_xy = [anfield[0].len(), anfield.len()];
    
    /* the piece top left corner position */
    let mut answer_xy = [i128::MIN, i128::MIN];
    /* the most argessive enemy cell position */
    
    
    //todo: implement.
    /* to default values for correct piece position */
    let mut fresh_calculation:bool = true;
    
    let piece_height = piece.len() as i128;
    let piece_width = piece[0].len() as i128;

    append_to_file(DEBUG_FILE, &format!("\n\n===\ninside find_position piece_height {} piece_width {}", piece_height,piece_width)).expect("cannot write to debug file");
    
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
    if self.major_strategy == MajorStrategy::FORK {/*agressively invade to fork sides */
      self.switch_fork_direction();
      
      let mut fork_strategy_still_effective = false; /*false - no more ways to increase the progress to fork directions*/
      let mut left_fork_strategy_still_effective = false;
      let mut right_fork_strategy_still_effective = false;
      
      /* use buffers to rollback finally not used variant
      * after choose which one to use,
      * left or right fork
      */
      let mut buffer_global_max_distance_left_fork =
      self.global_max_distance_proportion_left_fork.clone();
      
      let mut buffer_global_max_distance_right_fork =
      self.global_max_distance_proportion_right_fork.clone();
      
      let mut answer_xy_left_fork = answer_xy.clone();
      let mut answer_xy_right_fork = answer_xy.clone();
      
      for y in y_iterator.clone() {
        for x in x_iterator.clone() {
          if self.position_is_correct(anfield, piece, x, y, player_char){
            
            if fresh_calculation {/*use the first found correct position for the piece as default */
              fresh_calculation = false;
              answer_xy = [x, y].clone();
              answer_xy_left_fork = [x, y].clone();
              answer_xy_right_fork = [x, y].clone();
            }
            
            let max_distance_left_fork =
            self.find_most_agressive_distance_of_piece_cell(
              piece,
              [x,y].clone(),
              self.major_fork_left.clone(),
              &anfield_size_xy.clone()
            );
            
            let max_distance_right_fork =
            self.find_most_agressive_distance_of_piece_cell(
              piece,
              [x,y].clone(),
              self.major_fork_right.clone(),
              &anfield_size_xy.clone()
            );
            
            if max_distance_left_fork > buffer_global_max_distance_left_fork{
              buffer_global_max_distance_left_fork = max_distance_left_fork.clone();
              fork_strategy_still_effective = true;
              left_fork_strategy_still_effective = true;
              answer_xy_left_fork = [x, y].clone();
            }
            
            if max_distance_right_fork > buffer_global_max_distance_right_fork{
              buffer_global_max_distance_right_fork = max_distance_right_fork.clone();
              fork_strategy_still_effective = true;
              right_fork_strategy_still_effective = true;
              answer_xy_right_fork = [x, y].clone();
            }
            
          }
        }
      }
      
      if fork_strategy_still_effective
      {
        if self.fork_direction == ForkDirection::LEFT{
          
          if left_fork_strategy_still_effective{
            
            self.global_max_distance_proportion_left_fork = buffer_global_max_distance_left_fork.clone();
            return answer_xy_left_fork.clone();
            
          } else if right_fork_strategy_still_effective{
            
            self.global_max_distance_proportion_right_fork = buffer_global_max_distance_right_fork.clone();
            return answer_xy_right_fork.clone();
            
          }
          
        } else if self.fork_direction == ForkDirection::RIGHT{
          
          if right_fork_strategy_still_effective{
            
            self.global_max_distance_proportion_right_fork = buffer_global_max_distance_right_fork.clone();
            return answer_xy_right_fork.clone();
            
          } else if left_fork_strategy_still_effective{
            
            self.global_max_distance_proportion_left_fork = buffer_global_max_distance_left_fork.clone();
            return answer_xy_left_fork.clone();
            
          }
          
        }
        
        
        //todo old solution before separate left and right forks
        
        // if buffer_global_max_distance_left_fork < buffer_global_max_distance_right_fork
        // && self.fork_direction == ForkDirection::RIGHT
        // {
          //   self.global_max_distance_proportion_left_fork = buffer_global_max_distance_left_fork.clone();
          //   return answer_xy_left_fork;
          // } else if self.fork_direction == ForkDirection::LEFT {
            //   self.global_max_distance_proportion_right_fork = buffer_global_max_distance_right_fork.clone();
            //   return answer_xy_right_fork;
            // }
          }else {
            // self.major_strategy = MajorStrategy::SPEAR;
          }
          
        }
        
        
        // now if strategies did not work, then try to find the default answer
        
        
        append_to_file(DEBUG_FILE, &format!("\n====\nanswer_xy: {} {}", answer_xy[0], answer_xy[1])).expect("cannot write to debug file");
        answer_xy
      }
      
      /** 
      the piece position is correct if all(except one) non-empty cells of the piece
      are placed on the empty cells of the field, and only one non-empty cell
      of the piece is placed on the player cell(any cell covered by the
        player char by the player piece placement previously)
        */
        fn position_is_correct(
          &mut self,
          anfield: &VecDeque<VecDeque<char>>,
          piece: &VecDeque<VecDeque<char>>,
          x: i128, y: i128, player:&[char;2]
        ) -> bool {
          
          append_to_file(DEBUG_FILE, &format!("\n\n===\ninside position is correct")).expect("cannot write to debug file");
          // append_to_file(DEBUG_FILE, &format!("anfield {:?}" ,anfield)).expect("cannot write to debug file");
          append_to_file(DEBUG_FILE, &format!("piece {:?}" ,piece)).expect("cannot write to debug file");
          append_to_file(DEBUG_FILE, &format!("x {} y {}" ,x,y)).expect("cannot write to debug file");
          append_to_file(DEBUG_FILE, &format!("neg_x {} neg_y {}" ,self.piece_negative_xy[0],self.piece_negative_xy[1])).expect("cannot write to debug file");
          
          
          append_to_file(DEBUG_FILE, &format!("player {:?}" ,player)).expect("cannot write to debug file");
          
          /*
          only one cell from the piece must be placed on the player cell, so
          when the player_cells_hovered_by_piece is 1, for all the piece cells,
          the position is correct, otherwise it is not
          */
          let mut player_cells_hovered_by_piece:i128 = 0;
          
          /*manage negative indices */
          let zip_y = (y + self.piece_negative_xy[1] as i128) as usize;
          let zip_x = (x + self.piece_negative_xy[0] as i128) as usize;
          
          append_to_file(DEBUG_FILE, &format!("zip_x {} zip_y {}" ,zip_x,zip_y)).expect("cannot write to debug file");

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
          append_to_file(DEBUG_FILE, "the piece position is correct!!!\n\n").expect("cannot write to debug file");
          true /*the piece position is correct*/
        }
        
        pub fn reset_negative_xy(&mut self){
          self.piece_negative_xy = [usize::MIN, usize::MIN];
        }
        
      }