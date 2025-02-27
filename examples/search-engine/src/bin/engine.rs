use clap::Parser;
use search::index::InvertedIndex;
use search::loader::Loader;
use search::model::Boolean;
use search::Model;
use std::io;

pub fn launcher<M>(file_path: String, max_num_abs: usize, model: M)
where
    M: Model<InvertedIndex>,
{
    match Loader::from(file_path) {
        Ok(loader) => {
            let (_, index, collection) = loader.load(max_num_abs);
            loop {
                let mut query = String::new();
                println!("\n\nPlease enter a query, press Ctrl + C to exit");
                io::stdin()
                    .read_line(&mut query)
                    .expect("Failed to read query");
                let result = model.retrieve(query.as_str(), &index, &collection);
                println!("{:?}", result);
            }
        }
        Err(error) => println!("Error reading file,\n{error}"),
    };
}

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
    launcher(cli.file_abs_path, cli.max_num_abs, Boolean::new());
}
