use serde::{Serialize, Deserialize};
use serde_json::Error as JSONError;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use crate::task::{Task};
use crate::utils::err_no_task;

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub max_id: usize,
    pub count: usize,
    pub task_list: HashMap<usize, Task>,
}

impl<'a> TaskList {
    pub fn with_capacity(capacity: usize) -> TaskList {
        TaskList {
            max_id: 0,
            count: 0,
            task_list: HashMap::with_capacity(capacity),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> TaskList {
        let file: File = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return TaskList::with_capacity(10),
        };

        match serde_json::from_reader(file) {
            Ok(task_list) => task_list,
            Err(_) => TaskList::with_capacity(10),
        }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), JSONError> {
        let file = match File::create(&path) {
            Ok(file) => file,
            Err(e) => panic!(
                "Problem creating file {:#?}: {:#?}",
                path.as_ref().display(),
                e
            ),
        };

        serde_json::to_writer_pretty(file, self)
    }

    pub fn insert(&mut self, task: Task) -> Result<(), &'a str> {
        let task_id = task.id;

        if self.task_list.contains_key(&task_id) {
            return Err("Task with the same ID already exists");
        }

        self.task_list.insert(task_id, task);

        self.count += 1;
        self.max_id = self.max_id.max(task_id);

        Ok(())
    }

    pub fn remove(&mut self, id: usize) -> Result<(), &'a str> {
        if !self.task_list.contains_key(&id) {
            err_no_task(id);
        }

        match self.task_list.remove(&id) {
            Some(_) => {
                self.count -= 1;
                while self.max_id > 0 {
                    if self.task_list.contains_key(&self.max_id) {
                        break;
                    }
                    self.max_id -= 1;
                }

                Ok(())
            },
            None => Err("Could not remove task")
        }
    }

    pub fn get_mut(&mut self, index: &usize) -> &mut Task {
        self.task_list.get_mut(index).unwrap()
    }
}
