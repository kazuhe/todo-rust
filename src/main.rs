mod processes;
mod state;
mod todo;

use std::env;
use todo::enums::TaskStatus;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let title = &args[2];
    let state = state::read_file("./state.json");
    let status = match &state.get(title) {
        Some(result) => result.to_string().replace('\"', ""),
        None => "Pending".to_owned(),
    };
    let item = todo::todo_factory(title, TaskStatus::from_string(status));
    processes::process_input(item, command.to_string(), &state);
}
