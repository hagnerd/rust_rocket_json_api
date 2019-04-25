extern crate json_api;

use json_api::show_posts;

fn main() {
    let tasks = show_posts();

    println!("{} tasks being displayed", tasks.len());
    for task in tasks {
        println!("{}", task.description);
    }
}
