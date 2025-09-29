/*
1) Create todo
2) update todo
3) delect todo
4) get all todo
*/

use std::io::{self, Write};

#[derive(Debug)]
struct TasksStruck {
    id: u32,
    title: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<TasksStruck> = Vec::new();
    loop {
        let mut selected_oprations = String::new();

        println!("Hi! There welcome to cli todo");
        println!("Key for each oprations");
        println!("1) -> add Todo");
        println!("2) -> update task status");

        io::stdin()
            .read_line(&mut selected_oprations)
            .expect("Error during selected oprations");

        if selected_oprations.trim() == "1" {
            add_todo(&mut tasks);
        } else if selected_oprations.trim() == "2" {
            update_todo(&mut tasks);
        } else if selected_oprations.trim() == "3"  {
            delect_todo(&mut tasks);
        } else {
            println!("opration not supported");
            break;
        }
    }
}

fn add_todo(tasks: &mut Vec<TasksStruck>) {
    println!("Enter your first todo");

    loop {
        let mut task = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut task)
            .expect("Error during adding task");

        if task.trim().eq_ignore_ascii_case("done") {
            break;
        };

        if task.trim() == "break" {
            break;
        };

        if task.is_empty() {
            println!("Iqnore, It is empty");
            continue;
        };

        let task_obj = TasksStruck {
            id: tasks.len() as u32,
            title: task.trim().to_string(),
            done: false,
        };
        tasks.push(task_obj);
    }

    println!("all tasks is  {:#?}", tasks);
}

fn update_todo(tasks: &mut Vec<TasksStruck>) {

    if tasks.len() == 0 {
        println!("add atleast one task!");
    };

    println!("Enter the Id of which todo you want to edit ");
    println!("{:?}", tasks);
    let mut index   = String::new();

    io::stdin().read_line(&mut index).expect("error during update");

    let number: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
        println!("Invalid number!");
        return;
        }
    };

    let task = &mut tasks[number];

    task.done = true;
     
     println!("Done {:?}", task);
}

fn delect_todo (tasks: &mut Vec<TasksStruck>) {
    
    if tasks.len() == 0 {
        println!("add atleast one task!");
    };

    println!("Enter the Id of which todo you want to delect ");
    println!("{:?}", tasks);
    let mut index   = String::new();

    io::stdin().read_line(&mut index).expect("error during update");

    let number: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
        println!("Invalid number!");
        return;
        }
    };

    tasks.remove(number);    
     
     println!("Done {:?}", tasks);
}