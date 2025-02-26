use rocket::{get, launch, routes};
mod engine;

#[get("/boolean/search/<query>")]
async fn boolean_search(query: String) -> String {
    query
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![boolean_search])
}
