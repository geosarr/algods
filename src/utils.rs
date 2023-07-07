mod collinearity;
mod input_output;
mod point;
mod rand_vec_gen;

pub use collinearity::{BruteCollinearPoints, FastCollinearPoints};
pub use input_output::{read_lines, Reader, Reader2};
pub use point::{LineSegment, Point, Segment};
pub use rand_vec_gen::{gen_vec_rand_int, RandKind};
