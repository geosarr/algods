use algods::{utils::read_lines, utils::ThreeSum};
use clap::Parser;

#[derive(Parser)]
#[command(
    about = "\nFinds the number of triplets in a set of integers that sum up to zero. Integers should be unique !",
    long_about = "Solutions are counted in terms of triplets (or sets considering the integers to be different)."
)]
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
    let mut vec: Vec<isize> = Vec::new();
    if let Ok(lines) = read_lines(filename.as_str()) {
        for line in lines {
            if let Ok(row) = line {
                let values: Vec<isize> = row
                    .split(sep)
                    .map(|i| i.parse::<isize>().unwrap())
                    .collect();
                vec.extend_from_slice(&values);
            } else {
                panic!("bad row, check if the rows are correct.");
            }
        }
    } else {
        panic!("Error in file, check its content and the value of --sep")
    }

    let mut three_sum = ThreeSum::init(0, vec);
    println!("{}", three_sum.run());
}
