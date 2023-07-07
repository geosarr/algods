use algods::sort::InsertionSort;
use algods::utils::{Point, Segment};

#[derive(Debug)]
pub struct BruteCollinearPoints<T> {
    // List of Points
    vec: Vec<Point<T>>,
    // Line segments of 4 points
    seg: Option<Vec<Segment<T>>>,
}

impl<T> BruteCollinearPoints<T> {
    pub fn init(v: Vec<Point<T>>) -> Self {
        Self { vec: v, seg: None }
    }
}

impl<T: ToString + Ord + Clone> BruteCollinearPoints<T> {
    pub fn number_of_segments(&mut self) -> usize {
        match &self.seg {
            None => self.segments().len(),
            Some(lsegments) => lsegments.len(),
        }
    }

    pub fn segments(&mut self) -> Vec<Segment<T>> {
        // In this brute force version of finding all line segments
        // we check if 4 Points are collinear by checking if the slopes
        // between one of them and the rest are equal
        // run time complexity O(N^4)
        if self.seg.is_some() {
        } else {
            let mut v = Vec::<Segment<T>>::new();
            let n = self.vec.len();
            for i in 0..n {
                for j in i + 1..n {
                    let slope1 = self.vec[i].slope_to(&self.vec[j]);
                    for k in j + 1..n {
                        let slope2 = self.vec[i].slope_to(&self.vec[k]);
                        for l in k + 1..n {
                            let slope3 = self.vec[i].slope_to(&self.vec[l]);
                            // println!("{slope1}  {slope2}  {slope3}");
                            if slope1 == slope2 && slope2 == slope3 {
                                // self.vec[i], self.vec[j], self.vec[k], self.vec[l] are collinear,
                                // they are ordered with the self.compare_to(other)/partial_cmp order
                                // The extremal points will represent the line segment
                                let vec: Vec<&Point<T>> =
                                    vec![&self.vec[i], &self.vec[j], &self.vec[k], &self.vec[l]];
                                let m = InsertionSort::init(vec);
                                let vec = m.into_sorted_vec();
                                v.push(Segment::init(vec[0].clone(), vec[3].clone()));
                            }
                        }
                    }
                }
            }
            self.seg = Some(v);
        }
        self.seg.clone().expect("Failed to get Segments")
    }
}
