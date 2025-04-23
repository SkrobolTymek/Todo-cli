use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use home::home_dir;
use colored::Colorize;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    priority: u8,
    completed: bool,
    category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoApp {
    tasks: Vec<Task>,
    storage_path: PathBuf,
}

impl TodoApp {
    fn new() -> io::Result<Self> {
        let mut storage_path = home_dir().unwrap_or_else(|| PathBuf::from("."));
        storage_path.push(".todo-cli.json");
        
        let tasks = if storage_path.exists() {
            let mut file = File::open(&storage_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        Ok(Self {
            tasks,
            storage_path,
        })
    }

    fn save(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.storage_path)?;
        
        let json = serde_json::to_string_pretty(&self.tasks)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn add_task(&mut self, title: String, priority: u8, category: Option<String>) {
        self.tasks.push(Task {
            title,
            priority,
            completed: false,
            category,
        });
        self.save().expect("Failed to save tasks");
    }

    fn remove_task(&mut self, index: usize) -> Option<Task> {
        if index < self.tasks.len() {
            let task = self.tasks.remove(index);
            self.save().expect("Failed to save tasks");
            Some(task)
        } else {
            None
        }
    }

    fn complete_task(&mut self, index: usize) -> bool {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = true;
            self.save().expect("Failed to save tasks");
            true
        } else {
            false
        }
    }

    fn view_tasks(&self, filter: Option<&str>) {
        println!("\n{}", "Your Tasks:".bold().underline());
        println!("{:<5} {:<30} {:<10} {:<15} {:<10}", 
                 "#", "Task", "Priority", "Category", "Status");
        println!("{}", "-".repeat(80));

        for (i, task) in self.tasks.iter().enumerate() {
            if let Some(filter) = filter {
                if !task.title.contains(filter) && 
                   task.category.as_ref().map_or(true, |c| !c.contains(filter)) {
                    continue;
                }
            }

            let status = if task.completed {
                "✓".green()
            } else {
                "✗".red()
            };

            let priority = match task.priority {
                1 => "Low".blue(),
                2 => "Medium".yellow(),
                3 => "High".red(),
                _ => "Normal".normal(),
            };

            println!("{:<5} {:<30} {:<10} {:<15} {:<10}", 
                     i, 
                     task.title, 
                     priority, 
                     task.category.as_deref().unwrap_or("-"), 
                     status);
        }
    }

    fn clear_completed(&mut self) {
        self.tasks.retain(|task| !task.completed);
        self.save().expect("Failed to save tasks");
    }
}

fn main() -> io::Result<()> {
    let mut app = TodoApp::new()?;
    
    println!("{}", "Todo-CLI - Your terminal task manager".bold().green());
    println!("Version: {}\n", "1.0.0".bright_black());

    loop {
        println!("\n{}", "Main Menu:".bold().underline());
        println!("1. Add new task");
        println!("2. View tasks");
        println!("3. Complete task");
        println!("4. Remove task");
        println!("5. Clear completed tasks");
        println!("6. Search tasks");
        println!("7. Exit");

        print!("\n{}", "Select an option: ".bold());
        io::stdout().flush()?;
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("{}", "Task title: ".bold());
                io::stdout().flush()?;
                let mut title = String::new();
                io::stdin().read_line(&mut title)?;
                let title = title.trim().to_string();

                print!("{} (1-3): ", "Priority".bold());
                io::stdout().flush()?;
                let mut priority = String::new();
                io::stdin().read_line(&mut priority)?;
                let priority = priority.trim().parse().unwrap_or(2);

                print!("{} (optional): ", "Category".bold());
                io::stdout().flush()?;
                let mut category = String::new();
                io::stdin().read_line(&mut category)?;
                let category = if category.trim().is_empty() {
                    None
                } else {
                    Some(category.trim().to_string())
                };

                app.add_task(title, priority, category);
                println!("{}", "Task added successfully!".green());
            },
            "2" => {
                app.view_tasks(None);
            },
            "3" => {
                app.view_tasks(None);
                print!("\n{}", "Enter task number to complete: ".bold());
                io::stdout().flush()?;
                let mut num = String::new();
                io::stdin().read_line(&mut num)?;
                if let Ok(num) = num.trim().parse() {
                    if app.complete_task(num) {
                        println!("{}", "Task marked as completed!".green());
                    } else {
                        println!("{}", "Invalid task number!".red());
                    }
                }
            },
            "4" => {
                app.view_tasks(None);
                print!("\n{}", "Enter task number to remove: ".bold());
                io::stdout().flush()?;
                let mut num = String::new();
                io::stdin().read_line(&mut num)?;
                if let Ok(num) = num.trim().parse() {
                    if app.remove_task(num).is_some() {
                        println!("{}", "Task removed successfully!".green());
                    } else {
                        println!("{}", "Invalid task number!".red());
                    }
                }
            },
            "5" => {
                app.clear_completed();
                println!("{}", "Completed tasks cleared!".green());
            },
            "6" => {
                print!("{}", "Search term: ".bold());
                io::stdout().flush()?;
                let mut term = String::new();
                io::stdin().read_line(&mut term)?;
                app.view_tasks(Some(term.trim()));
            },
            "7" => {
                println!("{}", "Goodbye!".bold().blue());
                break;
            },
            _ => {
                println!("{}", "Invalid option, please try again.".red());
            }
        }
    }

    Ok(())
}
