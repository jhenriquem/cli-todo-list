mod input;
mod printer;
mod task;

use crate::task::Task;
use std::io::{self, Write};

fn main() {
    let mut list: Vec<Task> = Vec::new();

    loop {
        print!("\x1B[2J\x1B[1;1H");
        let counter = task::counter_completed_tasks(&list);
        printer::header(counter, list.len());

        for (i, task) in list.iter().enumerate() {
            printer::task(task, i);
        }

        print!("\n > ");
        io::stdout().flush().unwrap();
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("");

        let (exit, _message) = input::handler_input(&text, &mut list);

        if exit {
            break;
        }
    }
}
