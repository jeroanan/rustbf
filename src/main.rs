use std::{collections::HashMap};

mod machine_state;
mod read_program;
mod bf_config;

const PTR_INC: char = '>';
const PTR_DEC: char = '<';
const LOC_INC: char = '+';
const LOC_DEC: char = '-';
const LOOP_BEG: char = '[';
const LOOP_END: char = ']';
const PUT_CHAR: char = '.';

fn main() {

    let valid_instructions = [PTR_INC, PTR_DEC, LOC_INC, LOC_DEC, LOOP_BEG, LOOP_END, PUT_CHAR];

    let mut machine_state = machine_state::MachineState {
        memory: [0; bf_config::MEMORY_SIZE],
        mem_ptr: 0,
        program: ['\0'; 1001],
        program_ctr: 0,
        loops: [0; bf_config::MEMORY_SIZE],
        loop_ptr: 0,
    };

    type ValueFunction = fn(&mut machine_state::MachineState) -> bool;

    let mut char_map: HashMap<char, ValueFunction>= HashMap::new();
    char_map.insert(PTR_INC, inc_ptr);
    char_map.insert(PTR_DEC, dec_ptr);
    char_map.insert(LOC_INC, inc_loc);
    char_map.insert(LOC_DEC, dec_loc);
    char_map.insert(LOOP_BEG, loop_begin);
    char_map.insert(LOOP_END, loop_end);
    char_map.insert(PUT_CHAR, put_char);

    read_program::read_program_file(&mut machine_state);

    while machine_state.program[machine_state.program_ctr] != '\0' {
        let c = machine_state.program[machine_state.program_ctr];
        if let Some(func) = char_map.get(&c) {
            if func(&mut machine_state) {
                machine_state.program_ctr+=1;
            }
        } 
    }
}

fn inc_ptr(state: &mut machine_state::MachineState) -> bool {
    if state.mem_ptr < bf_config::MEMORY_SIZE  {
        state.mem_ptr+=1;
    } else {
        state.mem_ptr = 0;
    }

    return true;
}

fn dec_ptr(state: &mut machine_state::MachineState) -> bool {
    if state.mem_ptr > 0 {
        state.mem_ptr-=1;
    } else {
        state.mem_ptr = bf_config::MEMORY_SIZE;
    }

    return true;
} 

fn inc_loc(state: &mut machine_state::MachineState) -> bool {
    state.memory[state.mem_ptr]+=1;
    return true;
}

fn dec_loc(state: &mut machine_state::MachineState) -> bool {
    state.memory[state.mem_ptr]-=1;
    return true;
}

fn loop_begin(state: &mut machine_state::MachineState) -> bool {
    state.program_ctr+=1;
    state.loops[state.loop_ptr] = state.program_ctr;
    state.loop_ptr+=1;

    return false;
}

fn loop_end(state: &mut machine_state::MachineState) -> bool {
    if state.memory[state.mem_ptr] > 0 {
        state.program_ctr = state.loops[state.loop_ptr-1];
        return false;
    }
    state.loop_ptr-=1;

    return true;
}

fn put_char(state: &mut machine_state::MachineState) -> bool {
    if let Some(c) = char::from_u32(state.memory[state.mem_ptr] as u32) {
        print!("{}", c);
    }

    return true;
}
