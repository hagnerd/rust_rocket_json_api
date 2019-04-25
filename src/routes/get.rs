use rocket_contrib::json::Json;

use crate::models::Task;
use crate::db::{show_tasks, show_task};

#[get("/tasks")] 
pub fn tasks() -> Json<Vec<Task>> {
    Json(show_tasks())
}

#[get("/task/<id>")]
pub fn task_by_id(id: i32) -> Json<Task> {
    Json(show_task(id))
}

#[get("/")]
pub fn index() -> &'static str {
    "Hey there"
}
