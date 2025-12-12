use dialoguer::console::style;
use dialoguer::{Confirm, Input, Password, Select};
use serde_json::{self};
use std::error::Error;
use std::{
    fs::{self},
    path::Path,
};

#[derive(serde::Serialize, Debug, serde::Deserialize)]
struct PasswordSection {
    username: String,
    password: String,
}
#[derive(serde::Serialize, Debug, serde::Deserialize)]
struct Manager {
    master: String,
    passwords: Vec<PasswordSection>,
}

enum Action {
    Get,
    Add,
    Update,
    Delete,
    Exit,
}

impl Action {
    fn all() -> [&'static str; 5] {
        ["Get", "Add", "Update", "Delete", "Exit"]
    }

    fn from_index(idx: usize) -> Self {
        match idx {
            0 => Action::Get,
            1 => Action::Add,
            2 => Action::Update,
            3 => Action::Delete,
            _ => Action::Exit,
        }
    }
}

fn main() {
    if let Err(e) = run_cli() {
        eprintln!("Fatal error: {}", e);
        std::process::exit(1);
    }
}

fn run_cli() -> Result<(), Box<dyn Error>> {
    let path = Path::new("one.json");
    print_header();

    let mut manager = match get_json("one.json") {
        Ok(value) => {
            let input = Password::new()
                .with_prompt("Enter your 'Master Password' to continue")
                .interact()
                .unwrap();

            if input.trim() != value.master {
                eprintln!("{}", style("Incorrect master password!").red());
                return Ok(());
            }

            value
        }

        Err(_) => {
            let master = Password::new()
                .with_prompt("Create a 'Master Password' to continue")
                .with_confirmation("Confirm master password", "Passwords do not match")
                .interact()?;

            create_file(&master.trim(), path)?;
            println!("{}", style("Created password store").green().bold());
            Manager {
                master: master.trim().to_string(),
                passwords: Vec::new(),
            }
        }
    };

    loop {
        let selected_action = Select::new()
            .with_prompt("Select action")
            .items(&Action::all())
            .default(0)
            .interact()?;

        let action = Action::from_index(selected_action);

        match action {
            Action::Get => action_get(&manager),
            Action::Add => {
                action_add(&mut manager)?;
                update_file(&manager, path)?;
            }

            Action::Update => {
                if manager.passwords.is_empty() {
                    println!("{}", style("No entries to update").yellow());
                } else {
                    action_update(&mut manager)?;
                    update_file(&manager, path)?;
                }
            }
            Action::Delete => {
                if manager.passwords.is_empty() {
                    println!("{}", style("No entries to delete").yellow());
                } else {
                    action_delect(&mut manager)?;
                    update_file(&manager, path)?;
                }
            }
            Action::Exit => {
                println!("{}", style("Goodbye").cyan());
                break;
            }
        };
    }
    print_footer();
    Ok(())
}

fn action_get(manager: &Manager) {
    if manager.passwords.is_empty() {
        println!("{}", style("No passwords stored").yellow());
        return;
    }

    let uname_w = manager
        .passwords
        .iter()
        .map(|p| p.username.len())
        .max()
        .unwrap_or(6)
        .max("Username".len());
    let pass_w = "Password".len().max(8);

        // header
    println!(
        "{}  {}",
        style(format!("{:uname_w$}", "Username", uname_w = uname_w)).underlined(),
        style(format!("{:pass_w$}", "Password", pass_w = pass_w)).underlined()
    );

      for p in &manager.passwords {
        println!(
            "{:uname_w$}  {:}",
            style(&p.username).bold(),
            style(&p.username).bold()
        );
    }
}

fn action_add(manager: &mut Manager) -> Result<(), Box<dyn Error>> {
    let username: String = Input::new().with_prompt("enter username").interact_text()?;

    let password: String = Password::new()
        .with_prompt("Enter Password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact()?;

    manager
        .passwords
        .push(PasswordSection { username, password });
    println!("{}", style("Added successfully").green().bold());
    Ok(())
}

fn action_update(manager: &mut Manager) -> Result<(), Box<dyn Error>> {
    let mut choices: Vec<&str> = Vec::new();
    for value in &manager.passwords {
        choices.push(&value.username);
    }

    let idx = Select::new()
        .with_prompt("Select a value to update")
        .items(&choices)
        .interact()?;

    let new_password = Password::new()
        .with_prompt("Enter new password")
        .interact()?;

    manager.passwords[idx].password = new_password;

    println!("{}", style("Upated Successfuly").green().bold().cyan());
    Ok(())
}

fn action_delect(manager: &mut Manager) -> Result<(), Box<dyn Error>> {
    let choices: Vec<&str> = manager
        .passwords
        .iter()
        .map(|p| p.username.as_str())
        .collect();

    let delect_which_passoword = Select::new()
        .with_prompt("Select a value to update")
        .items(&choices)
        .interact()?;

    let ok = Confirm::new()
        .with_prompt(format!("Delete '{}' forever?", choices[delect_which_passoword]))
        .default(false)
        .interact()?;

    if ok {
        manager.passwords.remove(delect_which_passoword);
        println!("{}", style("🗑️  Deleted successfully").green());
    } else {
        println!("{}", style("Delete aborted").dim());
    }
    
    Ok(())
}

fn create_file(
    master_password: &str,
    path: impl AsRef<std::path::Path>,
) -> Result<(), std::io::Error> {
    if master_password.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "master passowrd cannot be empty",
        ));
    }

    let path = path.as_ref();

    let v = Manager {
        master: master_password.to_string(),
        passwords: Vec::new(),
    };

    let contents = serde_json::to_string(&v)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;

    fs::write(path, contents)?;

    Ok(())
}

fn get_json(path: impl AsRef<std::path::Path>) -> Result<Manager, Box<dyn std::error::Error>> {
    let message = fs::read_to_string(path)?;

    let json_value: Manager = serde_json::from_str(&message)?;

    Ok(json_value)
}

fn update_file(
    data: &Manager,
    file_path: impl AsRef<std::path::Path>,
) -> Result<(), std::io::Error> {
    let str_data = serde_json::to_string_pretty(&data)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    fs::write(file_path, str_data)?;

    Ok(())
}

fn print_header() {
    let title = style("┏━ Password Manager ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓").bold();
    let subtitle = style("Manage small secrets. Not a vault.").italic();
    println!("\n{}\n{}", title, subtitle);
    println!("{}", style("Hint: passwords are hidden by default.").dim());
    println!();
}

fn print_footer() {
    println!();
    println!(
        "{}",
        style("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛").dim()
    );
}
