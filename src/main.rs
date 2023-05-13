use std::collections::HashMap;

mod machine_state;
mod rom;
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

    let mut machine_state = machine_state::initialize_machine();

    type ValueFunction = fn(&mut machine_state::MachineState);

    let mut char_map: HashMap<char, ValueFunction>= HashMap::new();
    char_map.insert(PTR_INC, inc_ptr);
    char_map.insert(PTR_DEC, dec_ptr);
    char_map.insert(LOC_INC, inc_loc);
    char_map.insert(LOC_DEC, dec_loc);
    char_map.insert(LOOP_BEG, loop_begin);
    char_map.insert(LOOP_END, loop_end);
    char_map.insert(PUT_CHAR, put_char);

    let mut rom = rom::initialize_rom(&mut machine_state);

    rom.read_program_file(&mut machine_state);

    while machine_state.program[machine_state.get_program_counter()] != '\0' {
        let c = machine_state.program[machine_state.get_program_counter()];
        if let Some(func) = char_map.get(&c) {
            func(&mut machine_state);
            if !machine_state.should_skip_next_pc_step() {
                machine_state.step_program_counter();
            }
        } 
    }
}

fn inc_ptr(state: &mut machine_state::MachineState) {
    state.inc_mem_ptr();
}

fn dec_ptr(state: &mut machine_state::MachineState) {
    state.dec_mem_ptr();
} 

fn inc_loc(state: &mut machine_state::MachineState) {
    state.inc_mem_loc_val();
}

fn dec_loc(state: &mut machine_state::MachineState) {
    state.dec_mem_loc_val();
}

fn loop_begin(state: &mut machine_state::MachineState) {
    state.loop_begin();
}

fn loop_end(state: &mut machine_state::MachineState) {
    state.loop_end();
}

fn put_char(state: &mut machine_state::MachineState) {
    let c = state.get_char_to_display();
    print!("{}", c);
}
