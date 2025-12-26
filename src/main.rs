use std::{collections::HashMap, env, process,fs, error::Error, path::Path};


fn main() {
    let args: Vec<String> = env::args().collect();

    let args: Args = Args::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    let mut todos:HashMap<String,Task> = get_todo().unwrap_or_else(|err| {
        eprintln!("Problem reading todo: {}", err);
        process::exit(1);
    });

    parse_input(&mut todos, &args);

    save_todo(&todos).unwrap_or_else(|err| {
        eprintln!("Problem saving todo: {}", err);
        process::exit(1);
    })
}

fn get_todo() -> Result<HashMap<String,Task>, Box<dyn Error>> {
    if !Path::new("todo.json").exists() {
        return Ok(HashMap::new());
    }
    let todo_json = fs::read_to_string("todo.json")?;
    // println!("Todo before: {}", todo_json);
    let todo:HashMap<String,Task> = serde_json::from_str(&todo_json)?;
    Ok(todo)
}

fn save_todo(todo: &HashMap<String, Task>) -> Result<(),Box<dyn Error>>{
    let todo_json = serde_json::to_string_pretty(&todo).unwrap();
    // println!("Todo after: {}", todo_json);
    fs::write("todo.json",todo_json)?;
    Ok(())
}

#[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
struct Task {
    name: String,
    done: bool,
}

impl Task {
    fn new(name: &str) -> Task {
        Task {
            name: String::from(name),
            done: false,
        }
    }
}

struct Args {
    command: String,
    name: String,
}

impl Args {
    fn build(args: &[String]) -> Result<Args, &'static str> {
        //dbg!(args);
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let command = String::from(args[1].trim());
        if command != "add" && command != "list" && command != "done" && command != "delete" {
            return Err("Invalid Command");
        }

        let name = String::from(args[2].trim());

        Ok(Args { command, name })
    }
}

fn parse_input(todos: &mut HashMap<String, Task>, args: &Args) {
    match args.command.as_str() {
        "add" => {
            todos.insert(args.name.clone(), Task::new(&args.name));
            println!("Task {} added", args.name);
        }
        "list" => {
            if todos.len() == 0 {
                println!("There are no tasks");
                return ;
            }
            for value in todos.values() {
                println!("{} {}",value.name,if value.done {"✅"} else {"❌"});
            }
        }
        "done" => {
            if !todos.contains_key(&args.name){
                println!("There is no task with name {}",args.name);
                return ;
            }

            let task = todos.get_mut(&args.name).unwrap();
            task.done = true;
            println!("Task {} done", args.name);
        }
        "delete" => {
            let name = args.name.as_str();
            if !todos.contains_key(name){
                println!("There is no task with name {}",args.name);
                return ;
            }
            todos.remove(name);
            println!("Task {} deleted", args.name);
        }
        _ => {}
    }
}
