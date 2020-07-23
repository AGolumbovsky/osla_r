#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Behold the Placeholder!!!"
}

#[get("/elo")]
fn elo() -> &'static str {
    "elo you"
}

fn main() {
    rocket::ignite().mount("/elo", routes![index, elo]).launch();
}
