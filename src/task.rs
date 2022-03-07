use colored::{ColoredString, Colorize};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use crate::tags::Tags;

pub const PRIORITY_MIN: u8 = 1;
pub const PRIORITY_MAX: u8 = 5;
pub const PRIORITY_DEFAULT: u8 = 3;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub desc: String,
    pub tags: HashSet<String>,
    pub priority: u8,
    completed: bool,
    in_progress: bool,
}

impl Task {
    pub fn new(
        id: usize,
        title: &str,
        desc: &str,
        priority: Option<u8>
    ) -> Task {
        Task {
            tags: title.extract_tags(None)
                .union(&(desc.extract_tags(None))).cloned().collect(),
            id,
            title: title.to_owned(),
            desc: desc.to_owned(),
            priority: priority.unwrap_or(PRIORITY_DEFAULT),
            completed: false,
            in_progress: false
        }
    }

    pub fn is_completed(&self) -> bool { self.completed }

    pub fn is_in_progress(&self) -> bool { self.in_progress }

    pub fn start(&mut self) -> Result<(), &str> {
        if self.in_progress {
            return Err("Cannot start a task that is already in progress");
        }

        if self.completed {
            return Err("Cannot start a completed task");
        }

        self.in_progress = true;

        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), &str> {
        if self.completed {
            return Err("Cannot complete a task which is already completed");
        }

        if !self.in_progress {
            return Err("Cannot complete a task before starting it");
        }

        self.in_progress = false;
        self.completed = true;

        Ok(())
    }

    pub fn status_icon(&self) -> ColoredString {
        match self.completed {
            true => "✓".green(),
            false => match self.in_progress {
                true => "".cyan(),
                false => "✕".red(),
            }
        }.bold()
    }

    pub fn colored_id(&self) -> ColoredString {
        match self.priority {
            1 => format!("#{}", self.id).as_str().green(),
            2 => format!("#{}", self.id).as_str().cyan(),
            3 => format!("#{}", self.id).as_str().yellow(),
            4 => format!("#{}", self.id).as_str().magenta(),
            5 => format!("#{}", self.id).as_str().red(),
            _ => format!("#{}", self.id).as_str().blue(),
        }.bold()
    }

    pub fn print(&self, justify: usize, print_desc: bool) {
        println!(
            "\t{:<width$} | {} {}",
            self.colored_id(),
            self.status_icon(),
            self.title.as_str().highlight_tags(),
            width = justify
        );
        if print_desc {
            println!(
                "\t{:>width$}{}", "",
                self.desc.as_str().highlight_tags(),
                width = justify + 5
            );
        }
        println!()
    }

    pub fn priority_eq(&self, other: &Task) -> bool {
        self.id == other.id && self.priority == other.priority
    }

    pub fn priority_lt(&self, other: &Task) -> bool {
        if self.priority == other.priority {
            self.id < other.id
        } else {
            self.priority < other.priority
        }
    }

    pub fn priority_le(&self, other: &Task) -> bool {
        self.priority_eq(other) || self.priority_lt(other)
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{} | {} {}",
            self.colored_id(),
            self.status_icon(),
            self.title.as_str().highlight_tags()
        )
    }
}
