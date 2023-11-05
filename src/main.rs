use std::fs::{File, read_to_string};
use std::io::Write;
use std::env::{args, var_os};

fn main() {
    println!("");

    let binding = match var_os("HOME") {
        Some(dir) => Some(dir),
        None => match var_os("USERPROFILE") {
            Some(dir) => Some(dir),
            None => None
        }
    }.unwrap();
    let home_dir = binding.to_str().unwrap();
    let filename = format!("{}/todos", home_dir);

    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        println!("no arguments provided. use todo-cli-app help for help");
        return;
    }
    match args[1].as_str() {
        "a" | "add" => if args.len() > 2 {add_todo(args[2].clone(), &filename)} else {println!("nothing to add. please write your task after \"a\"/\"add\"")},
        "l" | "list" => list_todos(&filename),
        "d" | "delete" => if args.len() > 2 {delete_todo(args[2].clone(), &filename)} else {println!("nothing to remove. please write task number after \"d\"/\"delete\"")},
        "h" | "help" => get_help(),
        _ => println!("unknown argument provided. use todo-cli-app help for help")
    }
}

fn get_vector(path: &str) -> Vec<String> {
    File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("couldn't create file");
    
    read_to_string(path) 
        .unwrap()
        .lines()
        .map(String::from)  
        .collect()
}

fn write_file(contents: Vec<String>, message: String, path: &str) -> () {
    let mut f = File::options()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("cannot open file for writing");

    f.set_len(0)
        .expect("could not erase file contents");

    for v in contents {
        f.write_all((v + "\n").as_bytes()).expect("could not save todos");
    }
    
    println!("{}", message);
}

fn add_todo(task: String, path: &str) {
    let mut vector: Vec<String> = get_vector(path);

    vector.push(task.clone());

    write_file(vector, format!("successfully added \"{}\" to the todo list", task), path);
}

fn list_todos(path: &str) {
    let vector = get_vector(path);

    let mut count = 0;
    if vector.len() == 0 {
        println!("all clear!")
    } else {
        println!("todo list:");
        for v in vector {
            count += 1;
            println!("{}. {}", count, v)
        };
    };
}

fn delete_todo(index: String, path: &str) {
    let mut vector = get_vector(path);

    match index
            .trim()
            .parse::<usize>() {
                Ok(number) => {
                    if number - 1 >= vector.len() {
                        println!("no element with the index you provided");
                        return;
                    };
                    
                    let tasktext: String = vector[number - 1].clone();
                    vector.remove(number - 1);
                    write_file(vector, format!("successfully removed \"{}\" from the list", tasktext), path);

                },
                Err(_) => {
                    println!("your input is not an unsigned integer");
                }
            };
}

fn get_help() {
    println!(concat!(
        "todo app help message:\n",
        "a (add) \"[text]\" - add a task to the list. text has to be in quotes
        other arguments after text will be ignored\n",
        "l (list) - list tasks\n",
        "d (delete) [task number] - remove a task from the list
        other arguments after task number will be ignored\n",
        "h (help) - show this message"
    ))
}