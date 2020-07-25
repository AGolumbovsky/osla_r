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

fn main() {
    rocket::ignite()
    .mount("/", routes![index, word, learn])
    .launch();
}
