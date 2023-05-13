use std::fs;

use crate::bf_config;

pub struct Rom {
    program_code: [char; bf_config::MEMORY_SIZE+1],
}

impl Rom {

    pub fn read_program_file(&mut self) {

        let contents = fs::read_to_string(bf_config::FILE_NAME)
            .expect("Could not read file");

        let in_str = contents
            .split("\n")
            .collect::<Vec<&str>>();

        let joined = in_str.join("");

        let in_chars = joined.chars();

        for (i, c) in in_chars.enumerate() {
            if i < bf_config::MEMORY_SIZE {
                self.program_code[i] = c;
            }
        }
        self.program_code[bf_config::MEMORY_SIZE] = '\0';
    }

    pub fn get_code_at(&mut self, c: usize) -> char {
        return self.program_code[c];
    }
}

pub fn initialize_rom() -> Rom {
    let mut ret = Rom {
        program_code: ['\0'; bf_config::MEMORY_SIZE+1],
    };

    ret.read_program_file();

    return ret;
}