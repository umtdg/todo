use colored::Colorize;
use std::env::var;
use std::path::Path;
use std::process::exit;
use clap::ArgMatches;
use crate::task::{PRIORITY_MAX, PRIORITY_MIN, PRIORITY_DEFAULT};
use crate::tasklist::TaskList;

pub const ERR_SUCCESS: i32 = 0;
pub const ERR_SAVE: i32 = 1;
pub const ERR_INVALID_TASK_ID: i32 = 2;
pub const ERR_COMPLETE: i32 = 3;
pub const ERR_START: i32 = 4;

pub trait NumberOfDigits {
    fn number_of_digits(&self) -> usize;
}

impl NumberOfDigits for usize {
    fn number_of_digits(&self) -> usize {
        if self < &10 { 1 }
        else if self < &100 { 2 }
        else if self < &1000 { 3 }
        else if self < &10000 { 4 }
        else if self < &100000 { 5 }
        else if self < &1000000 { 6 }
        else if self < &10000000 { 7 }
        else if self < &100000000 { 8 }
        else if self < &1000000000 { 9 }
        else if self < &10000000000 { 10 }
        else { self.to_string().len() }
    }
}

pub fn remove_first(text: &str) -> &str {
    &text[1..text.len()]
}

pub fn print_error(msg: String) {
    println!("{} {}", "[!]".red().bold(), msg)
}

pub fn err_no_task(id: usize) {
    print_error(format!("Could not find task with ID {}", id));
    exit(ERR_INVALID_TASK_ID);
}

pub fn get_file_path() -> String {
    match var("TODO_FILE") {
        Ok(val) => val,
        Err(_) => match var("HOME") {
            Ok(home) => format!("{}/todo.json", home),
            Err(_) => panic!("Error reading environment variable HOME"),
        },
    }
}

pub fn get_task_id(matches: &ArgMatches) -> usize {
    let task_id: &str = matches.value_of("id").unwrap();
    match task_id.parse() {
        Ok(id) => id,
        Err(_) => {
            print_error(format!("Invalid task id: {}", task_id));
            exit(ERR_INVALID_TASK_ID);
        },
    }
}

pub fn get_priority(matches: &ArgMatches) -> u8 {
    if matches.occurrences_of("priority") == 1 {
        return match matches.value_of("priority").unwrap().parse() {
            Ok(p) => p,
            Err(_) => PRIORITY_DEFAULT,
        }.clamp(PRIORITY_MIN, PRIORITY_MAX);
    }

    return PRIORITY_DEFAULT;
}

pub fn save_and_exit<P: AsRef<Path>>(path: P, task_list: &TaskList) {
    match task_list.save_to_file(&path) {
        Ok(_) => println!("Saved to {:#?}", path.as_ref().display()),
        Err(e) => {
            print_error(format!("{:#?}", e));
            exit(ERR_SAVE);
        },
    }
    exit(ERR_SUCCESS);
}
