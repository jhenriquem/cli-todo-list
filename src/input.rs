use crate::task::{self, Task};

enum Commands {
    Update,
    Exit,
    Delete,
    New,
    Status,
    Unknown,
}

fn validate_command(cmd: &str) -> Commands {
    match cmd {
        "@update" => Commands::Update,
        "@exit" => Commands::Exit,
        "@status" => Commands::Status,
        "@new" => Commands::New,
        "@del" | "@delete" => Commands::Delete,
        _ => Commands::Unknown,
    }
}

pub fn handler_input(input: &str, list: &mut Vec<task::Task>) -> (bool, String) {
    let binding = input.trim().to_lowercase();
    let mut words = binding.split_whitespace();

    if let Some(cmd) = words.next() {
        let command = validate_command(cmd);

        match command {
            Commands::Exit => (true, "".to_string()),
            Commands::New => {
                let task_text = words.collect::<Vec<&str>>().join(" ");
                if !task_text.is_empty() {
                    task::add_task(list, &task_text);
                    (false, String::from("task added"))
                } else {
                    (false, String::from("task not added"))
                }
            }
            Commands::Delete => {
                if let Some(task_index) = words.next() {
                    match task::delete_task(list, task_index) {
                        true => (false, String::from("task deleted")),
                        false => (false, String::from("task no deleted")),
                    }
                } else {
                    (false, String::from("task no deleted"))
                }
            }
            Commands::Update => {
                if let Some(task_index) = words.next() {
                    let task_text = words.collect::<Vec<&str>>().join(" ");
                    let updated_task = Task {
                        text: task_text.to_string(),
                        status: task::Progress::Null,
                    };
                    if let Ok(index) = task_index.parse::<usize>() {
                        list[index] = updated_task;
                        (false, String::from("task updated"))
                    } else {
                        (false, String::from("task no updated"))
                    }
                } else {
                    (false, String::from("error"))
                }
            }

            Commands::Status => {
                if let Some(task_index) = words.next()
                    && let Some(status) = words.next()
                {
                    if let Ok(index) = task_index.parse::<usize>() {
                        list[index].status = task::validate_progress(status);
                        (false, String::from("task updated"))
                    } else {
                        (false, String::from("task no updated"))
                    }
                } else {
                    (false, String::from("error"))
                }
            }
            _ => (false, String::from("")),
        }
    } else {
        (false, String::from("error"))
    }
}
