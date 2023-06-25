use algods::{search::UnionFindAlgorithm, utils::PercolationStats};
use clap::Parser;

#[derive(Parser)]
#[command(
    about = "\nGives an estimate of the percolation transition probability using a Union Find algorithm",
    long_about = "A higher number of trials will give more robust confidence intervals."
)]
struct Cli {
    /// Size of the grid (number of rows or columns)
    #[arg(short, long)]
    grid_size: usize,

    /// Number of times percolation runs
    #[arg(short, long)]
    n_trials: usize,

    /// Union-Find algorithm
    #[arg(short, long)]
    algorithm: UnionFindAlgorithm,
}

fn main() {
    let cli = Cli::parse();

    println!("Running {}", cli.algorithm);
    let mut percol_stats =
        PercolationStats::with_capacity(cli.grid_size, cli.algorithm, cli.n_trials);
    percol_stats.compute();
    println!("\nmean = {}", percol_stats.mean());
    println!("unbiased standard deviation : {}", percol_stats.stddev());
    let ci = percol_stats.conf_interval();
    println!("confidence interval [{}, {}]", ci["low"], ci["up"]);
}
