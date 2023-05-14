use core::panic;
use std::fs;

use crate::bf_config;
use crate::bf_instructions;

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

        let mut ic = 0;

        for (_i, c) in in_chars.enumerate() {
            let is_valid = bf_instructions::VALID_INSTRUCTIONS.contains(&c);

            if is_valid && ic < bf_config::MEMORY_SIZE {
                self.program_code[ic] = c;
                ic+=1;
            }
        }

        self.program_code[bf_config::MEMORY_SIZE] = '\0';
    }

    pub fn get_code_at(&mut self, c: usize) -> char {
        return self.program_code[c];
    }

    pub fn get_loop_end_addr(&mut self, start_addr: usize) -> usize {
        if self.get_code_at(start_addr) != bf_instructions::LOOP_BEG {
            panic!("Invalid loop start address");
        }

        let mut loop_depth = 0;
        let mut current_addr = start_addr + 1;

        while current_addr < bf_config::MEMORY_SIZE {
            let c = self.get_code_at(current_addr);

            if c == bf_instructions::LOOP_BEG {
                loop_depth+=1;
            } else if c == bf_instructions::LOOP_END {
                if loop_depth == 0 {
                    return current_addr;
                } else {
                    loop_depth-=1;
                }
            }

            current_addr+=1;
        }

        panic!("Unable to find matching end of loop!");

    }

}

pub fn initialize_rom() -> Rom {
    let mut ret = Rom {
        program_code: ['\0'; bf_config::MEMORY_SIZE+1],
    };

    ret.read_program_file();

    return ret;
}