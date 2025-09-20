use crate::task::*;

pub fn header(completed_number: i32, tasks_number: usize) {
    println!("@taskbook [{completed_number}/{tasks_number}]")
}

pub fn task(task: &Task, index: usize) {
    let status_icon = return_icon_task(task);

    let i = index + 1;
    println!("{i}. {status_icon} {}", task.text);
}
