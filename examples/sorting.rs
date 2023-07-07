use algods::sort::{BinaryHeapSort, InsertionSort};
use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point<T> {
    x: T,
    y: T,
}
impl<T: Ord> Point<T> {
    pub fn compare_to(&self, other: &Self) -> Ordering {
        if self.y < other.y || (self.y == other.y && self.x < other.x) {
            Ordering::Less
        } else if self.x == other.x && self.y == other.y {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}
impl<T: Ord> Ord for Point<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare_to(other)
    }
}
impl<T: Ord> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn main() {
    let mut vec: Vec<isize> = vec![-1, -10, 15, 16, -18];
    let insertion = InsertionSort::<isize>::init(vec.clone());

    // sort using the insertion sort algorithm
    let insertion_sorted_vec = insertion.into_sorted_vec();
    println!("{:?}", insertion_sorted_vec);
    // sort using the standard library vec sort
    vec.sort();

    assert_eq!(insertion_sorted_vec, vec);

    // You can sort objects whose type is at least Clone and Ord
    // For example points in ZxZ implementing an Ord such that
    // points whith higher ordinates are larger, and two points
    // with the same ordinate will be ordered according to their
    // abscissa: (-1,0) < (0,0) < (-1, 1)

    let origin = Point { x: 0, y: 0 };
    let point_beyond_origin = Point { x: -1, y: 1 };
    let point_below_origin = Point { x: -1, y: 0 };
    let mut points = vec![origin, point_beyond_origin, point_below_origin];

    let heap = BinaryHeapSort::init(points.clone());
    let heap_sorted_vec = heap.into_sorted_vec();
    println!("{:?}", heap_sorted_vec);
    points.sort();

    assert_eq!(heap_sorted_vec, points);
}
