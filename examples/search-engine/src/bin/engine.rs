use clap::Parser;
use search::collection::Collection;
use search::loader::Loader;
use search::model::{Boolean, Phrase};
use search::{Index, Model};
use std::io;

pub fn loop_launcher<I, M>(model: M, index: I, collection: Collection)
where
    M: Model<I>,
{
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
pub fn launcher<I: Index, M>(file_path: String, max_num_abs: usize, model: M)
where
    M: Model<I>,
{
    match Loader::from(file_path) {
        Ok(loader) => {
            let (index, collection) = loader.load::<I>(max_num_abs);
            loop_launcher(model, index, collection);
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
    launcher(cli.file_abs_path, cli.max_num_abs, Phrase::new());
}
