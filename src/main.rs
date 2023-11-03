use std::fs::{File, read_to_string};
// use std::env::args;
use std::io::{stdin, stdout, Write};

static PATH: &str = "todos";

//? saasd

fn main() {
    println!("todo app omg\n");
    loop {
        println!(concat!("what you wanna do?\n",
            "1. (a) add a todo\n",
            "2. (l) list todos\n",
            "3. (d) delete a todo\n",
            "4. (q) nothing!! i wanna quit!!!!"
        ));

        print!("> ");
        stdout()
            .flush()
            .unwrap();
        let mut answer: String = String::new();
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

fn add_todo() {
    println!("what do you want to add?");

    print!("> ");
    stdout()
        .flush()
        .unwrap();
    let mut newtodo: String = String::new();
    stdin()
        .read_line(&mut newtodo)
        .expect("how tf did you manage to break this");

    let mut vector: Vec<String> = get_vector();

    vector.push(newtodo.replace("\n", ""));

    let mut f = 
    File::options()
        .write(true)
        .truncate(true)
        .open(PATH)
        .expect("cannot open file for writing");

    f.set_len(0)
        .expect("could not erase file contents");

    for v in vector {
        f.write_all((v + "\n").as_bytes()).expect("could not save todos");
    }

    println!("successfully added \"{}\" to the todo list\n", newtodo.replace("\n", ""));

}

fn list_todos() {
    let vector = get_vector();
    let mut count = 0;
    for v in vector {
        count += 1;
        println!("{}. {}", count, v)
    }
}

fn delete_todo() {
    let mut vector = get_vector();
    println!("enter number of a todo to remove");

    let mut del = String::new();
    print!("> ");
    stdout()
        .flush()
        .unwrap();
    stdin()
        .read_line(&mut del)
        .expect("huuuh");

    vector.remove(del.trim().parse().expect("not an integer"));
}