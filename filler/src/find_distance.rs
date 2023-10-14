use crate::find_::Finder;

impl Finder {
  pub fn find_distance(&mut self, xy1:[usize;2], xy2:[usize;2])-> f64 {
    let x1 = xy1[0] as f64;
    let y1 = xy1[1] as f64;
    let x2 = xy2[0] as f64;
    let y2 = xy2[1] as f64;
    ((x1-x2).powf(2.0) + (y1-y2).powf(2.0)).sqrt()
  }
}