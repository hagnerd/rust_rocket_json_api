#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod db;
pub mod routes;

use routes::{get, post};

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![get::tasks, get::index, get::task_by_id, post::new_task])
}
