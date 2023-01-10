use rocket::serde::{json::Json, Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    num::ParseIntError,
};

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    item: &'r str,
}

#[get("/")]
fn index() -> &'static str {
    "hello Rocket APP"
}

#[post("/addtask", data = "<task>")]
fn add_task(task: Json<Task<'_>>) -> &'static str {
    let mut tasks = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("Failed to add Task");
    let reader = BufReader::new(&tasks);
    let id = reader.lines().count();
    let task_item_str = format!("{}\n", task.item);
    let task_item_bytes = task_item_str.as_bytes();
    tasks
        .write(task_item_bytes)
        .expect("cannot write to tasks.txt");
    "Task added successfully"
}

#[get("/readtask")]
fn read_task() -> Json<Vec<String>> {
    let tasks = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("Unable to read tasks");

    let reader = BufReader::new(tasks);
    Json(
        reader
            .lines()
            .map(|line| {
                let line_string: String = line.expect("couldn't read line");
                let line_pieces: Vec<&str> = line_string.split(",").collect();
                line_pieces[1].to_string()
            })
            .collect(),
    )
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct TaskUpdate<'r> {
    id: u8,
    item: &'r str,
}

#[put("/edittask", data = "<task_update>")]
fn edit_task(task_update: Json<TaskUpdate<'_>>) -> &'static str {
    let tasks = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("Coudn't update task");
    let mut temp = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("temp.txt")
        .expect("Couldn't create temp file");

    let reader = BufReader::new(tasks);
    for line in reader.lines() {
        let line_string: String = line.expect("couldn't read line");
        let line_pieces: Vec<&str> = line_string.split(",").collect();

        if line_pieces[0]
            .parse::<u8>()
            .expect("unable to parse id as u8")
            == task_update.id
        {
            let task_items: [&str; 2] = [line_pieces[0], task_update.item];
            let task = format!("{}\n", task_items.join(","));
            temp.write(task.as_bytes())
                .expect("could not write to temp file");
        } else {
            let task = format!("{}\n", line_string);
            temp.write(task.as_bytes())
                .expect("could not write to temp file");
        }
    }

    std::fs::remove_file("tasks.txt").expect("unable to remove tasks.txt");
    std::fs::rename("temp.txt", "tasks.txt").expect("unable to rename temp.txt");
    "Task updated succesfully"
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct TaskId {
    id: u8,
}

#[delete("/deletetask", data = "<task_id>")]
fn delete_task(task_id: Json<TaskId>) -> &'static str {
    let tasks = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("unable to access tasks.txt");
    let mut temp = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("temp.txt")
        .expect("unable to access temp.txt");

    let reader = BufReader::new(tasks);

    for line in reader.lines() {
        let line_string: String = line.expect("could not read line");
        let line_pieces: Vec<&str> = line_string.split(",").collect();

        if line_pieces[0]
            .parse::<u8>()
            .expect("unable to parse id as u8")
            != task_id.id
        {
            let task = format!("{}\n", line_string);
            temp.write(task.as_bytes())
                .expect("could not write to temp file");
        }
    }

    std::fs::remove_file("tasks.txt").expect("unable to remove tasks.txt");
    std::fs::rename("temp.txt", "tasks.txt").expect("unable to rename temp.txt");
    "Task deleted succesfully"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![index, add_task, read_task, edit_task, delete_task],
    )
}
