use rocket::{get, launch, routes};
use search::{loader::Loader, model::Boolean};
mod engine;

#[get("/boolean/search/<query>")]
async fn boolean_search(query: String) -> String {
    let loader = Loader::from("enwiki-latest-abstract.xml.gz".to_string()).unwrap();
    let model = Boolean::new();
    let (index, collection) = loader.load(500);
    model
        .retrieve(&query, &index, &collection)
        .iter()
        .map(|doc| format!("{}\n\n", doc.content()))
        .take(100)
        .collect()
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![boolean_search])
}
