use algods::utils::{FastCollinearPoints, Point};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Absolute path to the file with the integers of the problem
    #[arg(short, long)]
    file_abs_path: String,

    /// Separator between the integers
    #[arg(short)]
    sep: char,
}

fn main() {
    let cli = Cli::parse();
    let filename = cli.file_abs_path;
    let sep = cli.sep;
    let points = Point::<isize>::from_file_to_vec(filename, sep);
    // println!("{:?} {:?}", points[0], points[1]);
    let mut collinear = FastCollinearPoints::<isize>::init(points);
    let segments = collinear.segments();
    println!("{}", segments.len());
    println!("{segments:?}");
    // println!("{}", segments[0].intersects(&segments[2]));
}
