use algods::utils::{LineSegment, Point, Segment};
// use std::collections::HashSet;

#[derive(Debug)]
pub struct FastCollinearPoints<T> {
    // List of Points
    vec: Vec<Point<T>>,
    // Line segments of 4 points
    seg: Option<Vec<Segment<T>>>,
}

impl<T> FastCollinearPoints<T> {
    pub fn init(v: Vec<Point<T>>) -> Self {
        // make sure that points are different
        Self {
            vec: v,
            seg: None::<Vec<Segment<T>>>,
        }
    }
}

impl<T: ToString + Ord + Clone> FastCollinearPoints<T> {
    pub fn number_of_segments(&mut self) -> usize {
        match &self.seg {
            None => self.segments().len(),
            Some(lsegments) => lsegments.len(),
        }
    }

    pub fn segments(&mut self) -> Vec<Segment<T>> {
        // run time complexity O(N^2)
        // space complexity O(N)
        if self.seg.is_some() {
        } else {
            let mut vec = Vec::<Segment<T>>::new();
            let n = self.vec.len();
            let mut vec_indices = (0..n).collect::<Vec<usize>>();
            // println!("{:?}", vec_indices);
            for k in 0..n {
                // build the lines passing throught the k^th point
                vec_indices.retain(|&e| e != k);
                // important to use LineSegment<T> here instead of Semgent<T>, because instances of
                // Segment<T> class are not ordered with slopes only.
                let mut lines_with_point_k = vec_indices
                    .iter()
                    .map(|&l| LineSegment::<T>::init(self.vec[k].clone(), self.vec[l].clone()))
                    .collect::<Vec<LineSegment<T>>>();
                vec_indices.push(k);
                // println!("{:?}", lines_with_point_k);

                // ordering the lines with respect to their slope
                lines_with_point_k.sort();
                // println!("{:?}", lines_with_point_k);

                // collinear points with point k are consecutive in lines_with_point_k
                let mut i = 0;
                while i < n - 1 {
                    let mut temp_vec = Vec::<&Point<T>>::new();
                    temp_vec.push(lines_with_point_k[i].p()); // point k
                    temp_vec.push(lines_with_point_k[i].q());
                    let mut l = i + 1;
                    while l < n - 1
                        && lines_with_point_k[i].slope() == lines_with_point_k[l].slope()
                    {
                        temp_vec.push(lines_with_point_k[l].q());
                        l += 1;
                    }
                    // println!("temp_vec = {:?}, l={}", temp_vec, l);
                    if temp_vec.len() >= 4 {
                        // a line of at least 4 points is found
                        temp_vec.sort();
                        let smaller_point = temp_vec[0].clone();
                        let largest_point = temp_vec.pop().unwrap().clone();
                        // using Segment<T> to avoid
                        vec.push(Segment::<T>::init(smaller_point, largest_point));
                    }
                    // println!("vec = {:?}", vec);
                    i = l;
                    // println!("\n");
                }
            }
            vec.sort();
            vec.dedup();
            self.seg = Some(vec);
        }
        self.seg.clone().expect("Failed to get Segments")
    }
}
