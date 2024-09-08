mod reminder;

use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}




fn main() {
    let matches = Command::new("Todo CLI")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Manage your todo list")
        .subcommand(
            Command::new("add")
                .about("Adds a new todo")
                .arg(Arg::new("title").required(true)),
        )
        .subcommand(Command::new("list").about("Lists all todos"))
        .subcommand(
            Command::new("complete")
                .about("Marks a todo as completed")
                .arg(Arg::new("id").required(true)),
        )
        .get_matches();

    let mut todos: Vec<Todo> = read_todos().unwrap_or_default();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let title = sub_matches.get_one::<String>("title").unwrap();
            let id = todos.len() as u32 + 1;
            todos.push(Todo {
                id,
                title: title.clone(),
                completed: false,
            });
            println!("Added todo: {}", title);
        }
        Some(("list", _)) => {
            for todo in &todos {
                println!(
                    "{}: {} [{}]",
                    todo.id,
                    todo.title,
                    if todo.completed { "x" } else { " " }
                );
            }
        }
        Some(("complete", sub_matches)) => {
            let id = sub_matches.get_one::<String>("id").unwrap().parse::<u32>().unwrap();
            if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
                todo.completed = true;
                println!("Completed todo: {}", todo.title);
            } else {
                println!("Todo with id {} not found", id);
            }
        }
        _ => {
            println!("Invalid command. Use --help for more information.");
        }
    }

    save_todos(&todos).unwrap();
}

fn read_todos() -> io::Result<Vec<Todo>> {
    let mut file = File::open("todos.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let todos: Vec<Todo> = serde_json::from_str(&contents)?;
    Ok(todos)
}

fn save_todos(todos: &[Todo]) -> io::Result<()> {
    let file = File::create("todos.json")?;
    let json = serde_json::to_string_pretty(todos)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}