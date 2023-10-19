use crate::find_::{Finder, Compas};

impl Finder{
  pub fn find_most_far_xy_opposite_the_direction(
    &mut self,
    anfield_size_xy:&[usize;2],
    direction:Compas,
  )-> [usize;2] {
    let asxy = anfield_size_xy;
    match direction {
      Compas::N => {[(asxy[0] - 1)/2, asxy[1] - 1]},
      Compas::S => {[(asxy[0] - 1)/2, 0]},
      Compas::W => {[asxy[0] - 1, (asxy[1] - 1)/2]},
      Compas::E => {[0, (asxy[1] - 1)/2]},
      Compas::NW => {[asxy[0] - 1, asxy[1] - 1]},
      Compas::NE => {[0, asxy[1] - 1]},
      Compas::SW => {[asxy[0] - 1, 0]},
      Compas::SE => {[0, 0]},
      Compas::CENTRAL => {[usize::MAX, usize::MAX]}, /* should never fire */
    }
  }
}