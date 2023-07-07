use algods::{
    sort::{BinaryHeapSort, InsertionSort},
    utils::Point,
};

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

    let origin = Point::<isize>::init(0, 0);
    let point_beyond_origin = Point::<isize>::init(-1, 1);
    let point_below_origin = Point::<isize>::init(-1, 0);
    let mut points = vec![origin, point_beyond_origin, point_below_origin];

    let heap = BinaryHeapSort::<Point<isize>>::init(points.clone());
    let heap_sorted_vec = heap.into_sorted_vec();
    println!("{:?}", heap_sorted_vec);
    points.sort();

    assert_eq!(heap_sorted_vec, points);
}
