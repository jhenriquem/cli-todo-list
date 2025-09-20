pub enum Progress {
    Null,
    Started,
    Completed,
}

pub struct Task {
    pub text: String,
    pub status: Progress,
}

pub fn validate_progress(progress: &str) -> Progress {
    match progress {
        "null" => Progress::Null,
        "started" => Progress::Started,
        "completed" => Progress::Completed,
        _ => Progress::Null,
    }
}

pub fn add_task(list: &mut Vec<Task>, task: &str) {
    let new_task = Task {
        text: String::from(task),
        status: Progress::Null,
    };

    list.push(new_task);
}

pub fn delete_task(list: &mut Vec<Task>, index: &str) -> bool {
    if let Ok(i) = index.parse::<usize>() {
        if i < list.len() {
            list.remove(i);
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn return_icon_task(task: &Task) -> String {
    match task.status {
        Progress::Null => String::from(" "),
        Progress::Started => String::from("󱋯 "),
        Progress::Completed => String::from(" "),
    }
}

pub fn counter_completed_tasks(list: &[Task]) -> i32 {
    let mut counter = 0;
    for task in list {
        match task.status {
            Progress::Completed => counter += 1,
            _ => continue,
        }
    }
    counter
}
