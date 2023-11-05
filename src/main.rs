use std::fs::{File, read_to_string};
use std::io::Write;
use std::env::args;

static PATH: &str = "todos";

fn main() {
    println!("");

    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        println!("no arguments provided. use todo-cli-app help for help");
        return;
    }
    match args[1].as_str() {
        "a" | "add" => if args.len() > 2 {add_todo(args[2].clone())} else {println!("nothing to add. please write your task after \"a\"/\"add\"")},
        "l" | "list" => list_todos(),
        "d" | "delete" => if args.len() > 2 {delete_todo(args[2].clone())} else {println!("nothing to remove. please write task number after \"d\"/\"delete\"")},
        "h" | "help" => get_help(),
        _ => println!("unknown argument provided. use todo-cli-app help for help")
    }
}

fn get_vector() -> Vec<String> {
    read_to_string(PATH) 
        .unwrap()
        .lines()
        .map(String::from)  
        .collect()
}

fn write_file(contents: Vec<String>, message: String) -> () {
    let mut f = File::options()
        .write(true)
        .truncate(true)
        .open(PATH)
        .expect("cannot open file for writing");

    f.set_len(0)
        .expect("could not erase file contents");

    for v in contents {
        f.write_all((v + "\n").as_bytes()).expect("could not save todos");
    }
    
    println!("{}", message);
}

fn add_todo(task: String) {
    let mut vector: Vec<String> = get_vector();

    vector.push(task.clone());

    write_file(vector, format!("successfully added \"{}\" to the todo list", task));
}

fn list_todos() {
    let vector = get_vector();

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

fn delete_todo(index: String) {
    let mut vector = get_vector();

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
                    write_file(vector, format!("successfully removed \"{}\" from the list", tasktext));

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