use std::io;

#[derive(Debug)]
struct Task {
    title: String,
    valuelvl: u32,
}




fn add_task(tasks: &mut Vec<Task>) {
    println!("Please input the name of the task:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("There was an error while reading the name");
    let name = name.trim().to_string();

    if name.is_empty() {
        println!("Task name cannot be empty.");
        return;
    }
    

    println!("Please input the importance of the task (NUMBER FROM 1- 5 PLEASE):");
    let mut importancy = String::new();
    io::stdin().read_line(&mut importancy).expect("There was an error while reading the importancy");
    match importancy.trim().parse::<u32>() {
        Ok(value) if value >= 1 && value <= 5 => {
            tasks.push(Task { title: name, valuelvl: value });
            println!("Task added!");
            println!("\nThe tasks: ");
            for task in tasks.iter() {
                println!("Task: {}, Importance: {} \n", task.title, task.valuelvl);
            }
        }
        Ok(_) => println!("Please enter a number between 1 and 5."),
        Err(_) => println!("Invalid number entered! Please enter a valid number."),
    }
    
   
}

fn remove_task(tasks: &mut Vec<Task>) {
    println!("Please input the name of the task to remove:");
    println!("\nThe tasks: ");
    for task in tasks.iter() {
        println!("Task: {}, Importance: {} \n", task.title, task.valuelvl);
    }
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("There was an error while reading the task name");
    let name = name.trim();

    if let Some(pos) = tasks.iter().position(|task| task.title == name) {
        tasks.remove(pos);
        println!("Task removed successfully.");
    } else {
        println!("No task with that name was found: TRY AGAIN") ;
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\nPress 1 to add a new task");
        println!("Press 2 to remove a task");
        println!("Press 3 to quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("There was an error while reading the choice");
        let choice = choice.trim();

        match choice {
            "1" => {
                add_task(&mut tasks);
            }
            "2" => {
                if tasks.is_empty() {
                    println!("No tasks to remove.");
                } else {
                    remove_task(&mut tasks);
                }
            }
            "3" => {
                println!("Are you sure you want to exit? y/n");
                let mut exitchoice = String::new();
                io::stdin().read_line(&mut exitchoice).expect("Reading exit choice error");
                let exitchoice = exitchoice.trim();
                match exitchoice{
                    "y"=>{
                        println!("Exiting..."); 
                        break;
                    }
                    "n"=>{
                        println!("Okey-dokey");
                    }
                    _=>{
                        println!("You chose a wrong option");
                    }
                }
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}
