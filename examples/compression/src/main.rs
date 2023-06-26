use algods::compression::RunLength;
use bitvec::prelude::Lsb0;
use clap::Parser;

#[derive(Parser)]
#[command(about = "\nCompresses and expands a file compression")]
struct Cli {
    /// Path to the file to compress
    #[arg(short, long)]
    path: String,
}
fn main() {
    let cli = Cli::parse();
    let run_length = RunLength::<u8, Lsb0>::from_file(&cli.path);
    let (comp, init_nb_bits) = run_length.compress();
    println!(
        "Initial number of bits = {init_nb_bits} vs Compressed number of bits = {}",
        comp.len(),
    );
    let original = run_length.expand(comp, init_nb_bits);
    // println!("{:?}", original);
    assert!(run_length.bits() == &original);
}
