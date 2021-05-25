#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
mod controllers;
mod types;

fn main() {
    rocket::ignite().mount("/", routes![
        controllers::create,
        controllers::read,
        controllers::update,
        controllers::delete
    ]).launch();
}
