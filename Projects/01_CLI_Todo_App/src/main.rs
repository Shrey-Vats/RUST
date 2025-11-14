use clap::{Parser, Subcommand};
use colored::Colorize;
use serde_json;
use serde::{Deserialize, Serialize};
use std::fs;
use tabled::{Tabled, Table};
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Remove { id: usize },
    Done { id: usize },
    Search { query: String}
}

#[derive(Debug)]
#[derive(Serialize,Deserialize)]
#[derive(Tabled)]
struct Todo {
    id: usize,
    todo: String,
    done: bool,
}
fn main() {
    let raw = match fs::read_to_string("info.json") {
        Err(err) => panic!("Error: {err}"),
        Ok(data) => data
    };

    let mut todo_list: Vec<Todo> = match serde_json::from_str(&raw) {
        Err(err) => panic!("Error: {err}"),
        Ok(todos) => todos
    };

 
    println!("\nWelcome to TodoMaster");

    let cli = Cli::parse();

    match cli.commands {
        Commands::Add { task } => add_todo(&mut todo_list, task),
        Commands::Done { id } => edit_todo(&mut todo_list, id),
        Commands::List => list_todos(&mut todo_list),
        Commands::Remove { id } => remove_todo(&mut todo_list, id),
        Commands::Search { query } => search_query(&mut todo_list, query),
    }
}

fn add_todo(todo_list: &mut Vec<Todo>, task: String) {

    todo_list.push(Todo {
        id: todo_list.len() + 1,
        todo: task.trim().to_string(),
        done: false,
    });

    println!("Added! Total todos: {}", todo_list.len());
    save_todo(todo_list);
}

fn remove_todo(todo_list: &mut Vec<Todo>, id: usize) {

    let index = match get_index(todo_list, id) {
        Err(err) => {
            eprintln!("error: {err}");
            return;
        },
        Ok(index) => index
    };

    if index >= todo_list.len() {
        eprintln!("Todo with ID {} not found", id);
        return;
    }

    todo_list.remove(index);

    for (i, t) in todo_list.iter_mut().enumerate() {
        t.id = i + 1
    }

    save_todo(todo_list);
}

fn list_todos(todo_list: &[Todo]) {

    if todo_list.is_empty() {
        println!("No Todo found");
        return;
    }

    let tabel =  Table::new(todo_list);

    println!("{}", tabel);
    // for t in todo_list.iter() {
    //       let status = if t.done { "✓ Done".green().bold() } else { "⏳ Pending".yellow() };
    //         println!("[{}] {}  ({})", t.id.to_string().blue(), t.todo, status);
    // }
}

fn get_index(todo_list: &mut Vec<Todo>, id: usize) -> Result<usize, String>{
    todo_list.iter().position(|x| x.id == id).ok_or_else(|| format!("Todo with id {}", id))
}

fn edit_todo(todo_list: &mut Vec<Todo>, id: usize) {
    
    let index =match get_index(todo_list, id)  {
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        },
        Ok(index) => index
    };


    if let Some(todo) = todo_list.get_mut(index) {
        if todo.done {
            todo.done = false;
            println!("Marked todo {} as pending!", id);
        } else {
            todo.done = true;
            println!("Marked todo {} as done!", id);
        };
    } else {
        eprintln!("Todo {} not found!", id);
    }

    save_todo(todo_list);
}

fn save_todo(todo_list: &mut Vec<Todo>) {

    let json = match serde_json::to_string_pretty(todo_list) {
        Err(err) => {
            eprintln!("Error: {err}");
            return;
        },
        Ok(v) => v
    };

    match fs::write("info.json", json) {
        Err(err) =>{ eprintln!("Error: {err}"); return;},
        Ok(_) => list_todos(todo_list),
    }
}

fn search_query(todo_list: &mut Vec<Todo>, query: String) {
 
  let q = query.to_lowercase();
  let results: Vec<&Todo> = todo_list.iter().filter(|t| t.todo.to_lowercase().contains(&q)).collect();

  if results.is_empty() {
    println!("No matching todos found.");
    return;
  }

  println!("{}", Table::new(results));
}