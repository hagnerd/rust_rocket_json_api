use super::schema::tasks;

use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub finished: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name="tasks"]
pub struct NewTask<'a> {
    pub description: &'a str,
}
