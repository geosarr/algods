use algods::utils::{BruteCollinearPoints, Point};

fn main() {
    let mut points = Vec::<Point<isize>>::new();
    for x in -1isize..2isize {
        for y in -1isize..2isize {
            points.push(Point::<isize>::init(x, y));
        }
    }
    points.push(Point::<isize>::init(2, 2));
    points.push(Point::<isize>::init(2, -2));
    points.push(Point::<isize>::init(2, 0));
    let mut brute_force = BruteCollinearPoints::<isize>::init(points);
    println!("{:?}", brute_force);
    // finding all segments of at least 4 collinear points (duplicates are possible)
    let segments = brute_force.segments();
    println!("{:#?}", segments);
}
