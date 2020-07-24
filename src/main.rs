#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Behold the Placeholder!!!"
}

#[get("/elo/<ass>")]
fn elo(ass: String) -> String {
    format!("elo, {}", ass.as_str())
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, elo])
    .launch();
}
