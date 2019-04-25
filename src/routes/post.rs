use rocket_contrib::json::Json;

use crate::models::{Task, NewTask};
use crate::db::{create_task};

#[post("/task", format="application/json", data="<task>")]
pub fn new_task(task: Json<NewTask>) -> Json<Task> {
   Json(create_task(task.description)) 
}
