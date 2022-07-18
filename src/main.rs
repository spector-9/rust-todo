mod to;
use crate::to::Todo;
use std::fs::{write, File, OpenOptions};
use std::io::prelude::*;
use std::io::ErrorKind;
use std::env::args;

fn get_input() -> Todo {

    let mut first: i8 = 0;
    let mut second: String = "".into();

    let number_of_argument = args().len();
    if number_of_argument != 4 {
        eprintln!("Invalid number of arguments");
        std::process::exit(1);
    }
    for i in 1..number_of_argument {
        match i {
            0 => (),
            1 => (),
            2 => {
                first = args()
                    .nth(i)
                    .expect("Invalid Entry")
                    .parse()
                    .expect("Please enter a number for priority");
                if first > 9 || first < 1 {
                    eprintln!("Priority must be in between 1 and 9");
                    std::process::exit(1);
                }
            }
            3 => second = args().nth(i).expect("Invalid Entry"),
            _ => panic!("Invalid number of arguments"),
        }
    }
    Todo::from(first, second)
}

fn create_todo_file() -> File {
    let path_to_todo_file: String =
        std::env::var("HOME").expect("$HOME not defined") + &"/.local/share/calcurse/todo";
    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&path_to_todo_file)
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(&path_to_todo_file).unwrap_or_else(|error| {
                    panic!("There was a problem creating a file: {}", error);
                })
            } else {
                panic!("Cannot read the file: {}", error);
            }
        });
    f
}

fn add_item_in_file() {
    let x = get_input().format_todo(true);
    let mut f = create_todo_file();
    write!(f, "{}\n", &x).unwrap();
}

fn delete_item_from_file(i: i32) {
    let path_to_todo_file: String =
        std::env::var("HOME").expect("$HOME not defined") + &"/.local/share/calcurse/todo";
    let f = File::open(&path_to_todo_file).expect("Cannot read file");
    let mut temp: Option<Vec<String>> = Some(vec![]);

    let line_buff = std::io::BufReader::new(f).lines();
    for (line_number, line_data) in line_buff.enumerate() {
        if line_number != i.try_into().expect("Please enter a positive number. Index start from 0") {
            temp.as_mut()
                .expect("Error in reading lines")
                .push(line_data.expect("Couldn't read lines"));
        }
    }
    let path_to_temp_todo_file = format!("{}{}", path_to_todo_file, ".temp");
    write(
        &path_to_temp_todo_file,
        &temp.expect("Can't write file").join("\n"),
    )
    .expect("Error while creating a temp file");
    std::fs::copy(&path_to_temp_todo_file, &path_to_todo_file).expect("Error in editing the file");
    std::fs::remove_file(&path_to_temp_todo_file).expect("Error deleting temp file");
}

fn main() {
    if args().nth(1) == Some("a".to_string()){
        add_item_in_file();
    }else if args().nth(1) == Some("d".to_string()) {
        delete_item_from_file(args().nth(2).expect("Please enter the line number to delete").parse().expect("Please enter a number"));
    }else if args().nth(1) == Some("p".to_string()) {
        delete_item_from_file(args().nth(2).expect("Please enter the line number to delete").parse().expect("Please enter a number"));
    }
}
