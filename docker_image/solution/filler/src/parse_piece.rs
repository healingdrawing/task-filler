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
      self.state = ParserState::GOT_PIECE;
      append_to_file(DEBUG_FILE, &format!("{:?}",self.piece)).expect("Unable to write data");
    }
  }

}