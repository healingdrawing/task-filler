use std::collections::VecDeque;
use crate::find_::Finder;



impl Finder {
  /** in time of development were detected the randomly generated pieces,
   * which have empty cells around the area of the not empty cells, which is
   * weird, and can affect the indexing when the piece iterated through the anfield.
   * Can not check properly can crap engine provided by 01-edu works properly
   * with negative indices for answer in case of the piece has empty cells around,
   * and can be placed in the corner of the anfield. Because no source code of engine,
   * and no documentation. It is just executable file. Why they hide the source of 
   * the engine if the engine just generate the pieces?
   * Smells like a scam.
   * And you can detect strange behavior of the engine, if send the very first
   * response answer with double new line at the end. It not only not raise error,
   * like it happens later started from step two and so on, but it is also autofix the
   * placement for the piece(detected it along vertical axis).
   * This happens with this map:
   * .....
   * ..$..
   * .....
   * ..@..
   * .....
   *
   * When you try to place the piece to x3 positons:
   * with \n\n at the end of string: "2 0\n\n" or "2 1\n\n" or "2 2\n\n",
   * the result was always the same, like you placed the piece to position "2 0\n".
   * 
   * Perhaps some extra parsing happens, and this can be some hidden key ,
   * to make something. Or it can be hidden key from the very strong robot
   * "terminator", which source code is hidden too. Hide robot source code
   * is reasonable, to hide solution, but not the engine. Not beautiful.
   */
  pub fn diagonal_of_the_piece_not_empty_cells_rectangle(&mut self, piece: &VecDeque<VecDeque<char>>)-> f64 {
    let mut min_ix = usize::MAX;
    let mut min_iy = usize::MAX;
    let mut max_ix = usize::MIN;
    let mut max_iy = usize::MIN;
    for y in 0..piece.len() {
      for x in 0..piece[0].len() {
        if piece[y][x] != '.' {
          if x < min_ix { min_ix = x; }
          if x > max_ix { max_ix = x; }
          if y < min_iy { min_iy = y; }
          if y > max_iy { max_iy = y; }
        }
      }
    }
    
    /*number of not empty rows */ let h = max_iy - min_iy +1;
    /*number of not empty columns */ let w = max_ix - min_ix +1;
    /*if something wrong */ if h < 1 || w < 1 { return 0.0; }
    /*diagonal */ ((h.pow(2) + w.pow(2)) as f64).sqrt()
  }

}