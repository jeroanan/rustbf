use std::fs;

use crate::machine_state;

const FILE_NAME: &'static str = "test1.txt";

pub fn read_program_file(state: &mut machine_state::MachineState) {

    let contents = fs::read_to_string(FILE_NAME)
        .expect("Could not read file");

    let in_str = contents
        .split("\n")
        .collect::<Vec<&str>>();

    let joined =  in_str.join("");

    let in_chars = joined.chars();

    for (i, c) in in_chars.enumerate() {
        if i > 999 {
            break;
        }
        state.program[i] = c;
    }
    state.program[1000] = '\0';
}