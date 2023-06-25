use clap::Parser;
use search::loader::Loader;
use search::model::Boolean;
use std::io;

#[derive(Parser)]
#[command(about = "\nSearches among english Wikipedia abstracts.")]
struct Cli {
    /// Absolute path to the xml.gz file containing the abstracts
    #[arg(short, long)]
    file_abs_path: String,

    /// Max number of abstracts to search on
    #[arg(short, long)]
    max_num_abs: usize,
}

fn main() {
    let cli = Cli::parse();
    let loader = Loader::init(cli.file_abs_path);
    let (index, collection) = loader.load(cli.max_num_abs);
    let model = Boolean::new();
    loop {
        let mut query = String::new();

        println!("\nPlease enter a query, press Ctrl + C to exit");

        io::stdin()
            .read_line(&mut query)
            .expect("Failed to read query");

        println!("{:?}", model.retrieve(query.as_str(), &index, &collection));
    }
}
