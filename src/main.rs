use std::collections::HashMap;

mod bf_config;
mod bf_instructions;
mod machine_state;
mod rom;

fn main() {

    let mut machine_state = machine_state::initialize_machine();

    type ValueFunction = fn(&mut machine_state::MachineState);

    let mut char_map: HashMap<char, ValueFunction>= HashMap::new();
    char_map.insert(bf_instructions::PTR_INC, inc_ptr);
    char_map.insert(bf_instructions::PTR_DEC, dec_ptr);
    char_map.insert(bf_instructions::LOC_INC, inc_loc);
    char_map.insert(bf_instructions::LOC_DEC, dec_loc);
    char_map.insert(bf_instructions::LOOP_BEG, loop_begin);
    char_map.insert(bf_instructions::LOOP_END, loop_end);
    char_map.insert(bf_instructions::PUT_CHAR, put_char);

    let mut rom = rom::initialize_rom();

    rom.read_program_file();

    while rom.get_code_at(machine_state.get_program_counter()) != '\0' {
        let c = rom.get_code_at(machine_state.get_program_counter());
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
