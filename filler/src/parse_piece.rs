use std::collections::VecDeque;

use crate::{parse_::{Parser, ParserState}, debug::{append_to_file, DEBUG_FILE}};

impl Parser {
  pub fn parse_piece_size(&mut self, input_line: &str){
    if input_line.starts_with("Piece "){
      let parts: Vec<&str> = input_line
      .trim_start_matches("Piece ")
      .trim_end_matches(':')
      .split_whitespace()
      .collect();
      
      // Parse the parts into usize
      if let [Ok(size1), Ok(size2)] =
      [parts[0].parse::<usize>(), parts[1].parse::<usize>()] {
        // Create an array with the extracted sizes
        self.piece_size = [size1, size2];
        
        // Print the array
        append_to_file(DEBUG_FILE, &format!("{:?}",self.piece_size)).expect("Unable to write data");
      } else {
        println!("Failed to parse sizes");
      }
      self.state = ParserState::GOT_PIECE_SIZE;
    }
  }
  
  pub fn parse_piece(&mut self, input_line: &str){
    
    let mut row: VecDeque<char> = VecDeque::new();
    for c in input_line.chars() {
      row.push_back(c);
    }
    self.piece.push_back(row);
    
    if self.piece.len() == self.piece_size[1] {

      append_to_file(DEBUG_FILE, &format!("piece before trim\n{:?}",self.piece)).expect("Unable to write data");

      self.trim_empty_rows_and_columns_from_the_end();//works, looks like not crushes
      self.trim_empty_rows_and_columns_from_the_beginning_and_fill_negative_xy();//debug danger it craps the process
      
      self.state = ParserState::GOT_PIECE;
      append_to_file(DEBUG_FILE, &format!("piece after trim\n{:?}",self.piece)).expect("Unable to write data");
      
    }
  }
  
  fn trim_empty_rows_and_columns_from_the_end(&mut self){
    // cut empty rows from the end
    while self.piece.back().unwrap().iter().all(|&x| x == '.') {
      self.piece.pop_back();
    }
    // cut empty columns from the end
    while self.piece.iter().all(|row| row.back().unwrap() == &'.') {
      for row in &mut self.piece {
        row.pop_back();
      }
    }
  }

  fn trim_empty_rows_and_columns_from_the_beginning_and_fill_negative_xy(&mut self){
    self.piece_negative_xy = [0, 0];
    // cut empty rows from the beginning
    while self.piece.front().unwrap().iter().all(|&x| x == '.') {
      self.piece.pop_front();
      self.piece_negative_xy[1] += 1;
    }
    // cut empty columns from the beginning
    while self.piece.iter().all(|row| row.front().unwrap() == &'.') {
      for row in &mut self.piece {
        row.pop_front();
      }
      self.piece_negative_xy[0] += 1;
    }
  }
  
}