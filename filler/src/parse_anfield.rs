use std::collections::VecDeque;

use crate::{parse_::{Parser, ParserState}, debug::{append_to_file, DEBUG_FILE}};

impl Parser {
  pub fn parse_anfield_size(&mut self, input_line: &str){
    if input_line.starts_with("Anfield "){
      let parts: Vec<&str> = input_line
      .trim_start_matches("Anfield ")
      .trim_end_matches(':')
      .split_whitespace()
      .collect();
      
      // Parse the parts into usize
      if let [Ok(size1), Ok(size2)] =
      [parts[0].parse::<usize>(), parts[1].parse::<usize>()] {
        // Create an array with the extracted sizes
        self.anfield_size = [size1, size2];
        
        // Print the array
        // append_to_file(DEBUG_FILE, &format!("{:?}",self.anfield_size)).expect("Unable to write data");
      } else {
        println!("Failed to parse sizes");
      }
      self.state = ParserState::GOT_ANFIELD_SIZE;
    }
  }
  
  pub fn parse_anfield(&mut self, input_line: &str){
    
    let mut row: VecDeque<char> = VecDeque::new();
    for c in input_line.chars() {
      row.push_back(c);
    }
    self.anfield.push_back(row);
    
    if self.anfield.len() > self.anfield_size[1] {
      self.state = ParserState::GOT_ANFIELD;
      self.anfield.pop_front();
      // cut first x4 items for each row
      for row in &mut self.anfield {
        for _ in 0..4 {
          row.pop_front();
        }
      }
      // append_to_file(DEBUG_FILE, &format!("{:?}",self.anfield)).expect("Unable to write data");
    }
  }
  
}