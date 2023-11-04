use std::fs::{File, read_to_string};
use std::io::{stdin, stdout, Write};

static PATH: &str = "todos";

fn main() {
    println!("== TODO APP OMG ==\n");

    loop {
        println!(concat!("what you wanna do?\n",
            "1. (a) add a todo\n",
            "2. (l) list todos\n",
            "3. (d) delete a todo\n",
            "4. (q) nothing!! i wanna quit!!!!"
        ));

        let mut answer: String = String::new();

        print!("> ");
        stdout()
            .flush()
            .unwrap();
        stdin().read_line(&mut answer)
            .expect("stupid");

        match answer.replace("\n", "").as_ref() {
            "1" | "a" => add_todo(),
            "2" | "l" => list_todos(),
            "3" | "d" => delete_todo(),
            "4" | "q" => break,
            &_ => println!("{}", answer)
        }
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

fn add_todo() {
    println!("\nwhat do you want to add?");

    let mut newtodo: String = String::new();

    print!("ADD: ");
    stdout()
        .flush()
        .unwrap();
    stdin()
        .read_line(&mut newtodo)
        .expect("how did you manage to break this");

    let mut vector: Vec<String> = get_vector();

    vector.push(newtodo.replace("\n", ""));

    write_file(vector, String::from("a"));

    println!("successfully added \"{}\" to the todo list\n", newtodo.replace("\n", ""));
}

fn list_todos() {
    let vector = get_vector();

    let mut count = 0;
    if vector.len() == 0 {
        println!("\nall clear!")
    } else {
        println!("\ntodo list:");
        for v in vector {
            count += 1;
            println!("{}. {}", count, v)
        };
    };
    println!("");
}

fn delete_todo() {
    let mut vector = get_vector();

    let mut count = 0;
    println!("\ntodo list:");
    for v in vector.clone() {
        count += 1;
        println!("{}. {}", count, v)
    };

    println!("\nenter number of a todo to remove");

    loop {
        let mut del = String::new();

        print!("DEL: ");
        stdout()
            .flush()
            .unwrap();
        stdin()
            .read_line(&mut del)
            .expect("huuuh");

        match del
            .trim()
            .parse::<usize>() {
                Ok(index) => {
                    if index - 1 >= vector.len() {
                        println!("no element with the index you provided\n");
                        continue;
                    };
                
                    vector.remove(index - 1);
                    write_file(vector, format!("successfully removed \"{}\" from the list\n", del.replace("\n", "")));

                    break;
                },
                Err(_) => {
                    println!("your input is not an unsigned integer");
                }
            };
    };
}
