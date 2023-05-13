use std::fs;

use crate::bf_config;
use crate::machine_state;

pub struct Rom {
    program_code: [char; bf_config::MEMORY_SIZE+1],
}

impl Rom {

    pub fn read_program_file(&mut self, state: &mut machine_state::MachineState) {

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
        self.program_code = state.program;
    }
}

pub fn initialize_rom(state: &mut machine_state::MachineState) -> Rom {
    let mut ret = Rom {
        program_code: ['\0'; bf_config::MEMORY_SIZE+1],
    };

    ret.read_program_file(state);

    return ret;
}