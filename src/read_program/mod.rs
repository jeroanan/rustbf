use std::fs;

use crate::bf_config;
use crate::machine_state;

pub fn read_program_file(state: &mut machine_state::MachineState) {

    let contents = fs::read_to_string(bf_config::FILE_NAME)
        .expect("Could not read file");

    let in_str = contents
        .split("\n")
        .collect::<Vec<&str>>();

    let joined = in_str.join("");

    let in_chars = joined.chars();

    for (i, c) in in_chars.enumerate() {
        if i < bf_config::MEMORY_SIZE {
            state.program[i] = c;
        }
    }
    state.program[bf_config::MEMORY_SIZE] = '\0';
}