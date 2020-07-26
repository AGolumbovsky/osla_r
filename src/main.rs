#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Behold the Placeholder!!!"
}

#[get("/<word>")]
fn word(word: String) -> String {
    format!("elo, {}", word.as_str())
}

#[get("/learn")]
fn learn() -> String {
    format!("Learn THIS for the time being")
}

//testing dis
use rocket::Request;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Where do you think you're going? Here?: {}", req.uri())
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, word, learn])
    .register(catchers![not_found])
    .launch();
}
