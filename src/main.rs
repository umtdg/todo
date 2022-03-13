mod task;
mod utils;
mod tags;
mod tasklist;

use clap::{Arg, Command};
use std::process::exit;
use itertools::Itertools;
use crate::task::Task;
use crate::tasklist::TaskList;
use crate::utils::{
    ERR_COMPLETE, ERR_START,
    NumberOfDigits,
    print_error,
    err_no_task,
    get_priority, get_file_path, get_task_id,
    save_and_exit,
};

fn base_command<'a>(name: &'a str, about: &'a str) -> Command<'a> {
    Command::new(name)
        .short_flag(name.chars().next().unwrap())
        .long_flag(name)
        .about(about)
}

fn main_command<'a>(
    name: &'a str,
    author: &'a str,
    about: &'a str,
    version: &'a str
) -> Command<'a> {
    Command::new(name)
        .author(author)
        .about(about)
        .version(version)
        .arg_required_else_help(true)
        .subcommand_required(true)
}

fn base_arg<'a>(name: &'a str, help: &'a str) -> Arg<'a> {
    Arg::new(name)
        .short(name.chars().next().unwrap())
        .long(name)
        .help(help)
}

fn required_arg<'a>(name: &'a str, help: &'a str) -> Arg<'a> {
    base_arg(name, help).required(true)
}

fn required_arg_with_value<'a>(name: &'a str, help: &'a str) -> Arg<'a> {
    required_arg(name, help).takes_value(true)
}

fn optional_arg<'a>(name: &'a str, help: &'a str) -> Arg<'a> {
    base_arg(name, help).required(false)
}

fn optional_arg_with_value<'a>(name: &'a str, help: &'a str, default: &'a str) -> Arg<'a> {
    optional_arg(name, help).takes_value(true).default_value(default)
}

fn main() {
    let todo_path: String = get_file_path();
    let mut task_list: TaskList = TaskList::from_file(&todo_path);

    let arg_task_id: Arg = Arg::new("id")
        .short('t')
        .long("task-id")
        .help("Id of the task")
        .takes_value(true)
        .required(true);

    // Main command
    let todo = main_command(
        "todo", "Umut DAG",
        "Simple todo manager",
        "1.0.0"
    );

    // Subcommands
    let subcmd_add = base_command("add", "Add a new task")
        .arg(required_arg_with_value(
            "title",
            "Title of the task"
        ))
        .arg(optional_arg_with_value(
            "priority",
            "Priority of the task between 1-5",
            "2"
        ))
        .arg(optional_arg_with_value(
            "desc",
            "Description of the task",
            ""
        ));
    let todo = todo.subcommand(subcmd_add);

    let subcmd_remove = base_command("remove", "Remove a task")
        .alias("rm")
        .arg(&arg_task_id);
    let todo = todo.subcommand(subcmd_remove);

    let subcmd_complete = base_command("complete", "Complete a task")
        .arg(&arg_task_id);
    let todo = todo.subcommand(subcmd_complete);

    let subcmd_start = base_command("start", "Start a task")
        .arg(&arg_task_id);
    let todo = todo.subcommand(subcmd_start);

    let subcmd_list = base_command("list", "List tasks")
        .alias("ls")
        .arg(base_arg("all", "Print all tasks"))
        .arg(base_arg("done", "Print only completed tasks"))
        .arg(base_arg("started", "Print only tasks which are in progress"))
        .arg(base_arg("verbose", "Also print task descriptions"));
    let todo = todo.subcommand(subcmd_list);

    let matches = todo.get_matches();

    match matches.subcommand() {
        Some(("add", matches)) => {
            let priority: u8 = get_priority(&matches);
            let id = task_list.max_id + 1;
            match task_list.insert(Task::new(
                id,
                matches.value_of("title").unwrap(),
                matches.value_of("desc").unwrap(),
                Some(priority)
            )) {
                Ok(_) => save_and_exit(&todo_path, &task_list),
                Err(e) => print_error(e.to_owned()),
            };
        },
        Some(("remove", matches)) => {
            let task_id: usize = get_task_id(matches);
            match task_list.remove(task_id) {
                Ok(_) => save_and_exit(&todo_path, &task_list),
                Err(e) => print_error(e.to_owned()),
            }
        },
        Some(("complete", matches)) => {
            let task_id: usize = get_task_id(matches);
            if !task_list.task_list.contains_key(&task_id) {
                err_no_task(task_id);
            }

            let task: &mut Task = task_list.get_mut(&task_id);

            match task.complete() {
                Ok(_) => {
                    println!("Completed task {}", task);
                    save_and_exit(&todo_path, &task_list);
                },
                Err(e) => {
                    print_error(e.to_owned());
                    exit(ERR_COMPLETE);
                },
            }
        },
        Some(("list", matches)) => {
            let all: bool = matches.occurrences_of("all") > 0;
            let done: bool = matches.occurrences_of("done") > 0;
            let started: bool = matches.occurrences_of("started") > 0;
            let print_desc: bool = matches.occurrences_of("verbose") > 0;
            let mut no_print: bool = true;

            println!();
            if task_list.count > 0 {
                let sorted = task_list.task_list.iter()
                    .filter(|task| {
                        let completed: bool = task.1.is_completed();
                        let in_progress: bool = task.1.is_in_progress();
                        all || (completed && done)
                        || (in_progress && started)
                        || (!completed && !done && !started)
                    })
                    .sorted_by(
                        |a, b| a.1.priority_cmp(b.1).reverse()
                    );
                for task_id in sorted {
                    let width = task_list.max_id.number_of_digits() + 1;
                    task_list.task_list[task_id.0].print(width, print_desc);
                    no_print = false;
                }
            }

            if no_print {
                println!("Nothing to do");
            }
        },
        Some(("start", matches)) => {
            let task_id: usize = get_task_id(&matches);
            if !task_list.task_list.contains_key(&task_id) {
                err_no_task(task_id);
            }

            let task: &mut Task = task_list.get_mut(&task_id);

            match task.start() {
                Ok(_) => {
                    println!("Started task {}", task);
                    save_and_exit(&todo_path, &task_list);
                },
                Err(e) => {
                    print_error(e.to_owned());
                    exit(ERR_START);
                },
            };
        },
        _ => unreachable!(),
    };
}
