use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::schema;
use crate::models::{Task, NewTask};

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn show_tasks() -> Vec<Task> {
    use schema::tasks::dsl::*;

    let connection = establish_connection();
    tasks.load::<Task>(&connection).expect("Error loading tasks")
} 

pub fn show_task(taskid: i32) -> Task {
    use schema::tasks::dsl::*;

    let connection = establish_connection();

    tasks.filter(id.eq(taskid)).first(&connection).unwrap()
}

pub fn create_task<'a>(description: &'a str) -> Task {
    let connection = establish_connection();

    use schema::tasks;

    let new_task = NewTask {
        description
    };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .get_result(&connection)
        .expect("Error saving new task")
}
