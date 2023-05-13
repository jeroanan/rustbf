use std::collections::HashMap;
use std::fs;

type BfMemMap = [usize; 1000];

struct MachineState {
    memory: BfMemMap,
    mem_ptr: usize,
    program: [char; 1001],
    program_ctr: usize,
    loops: [usize; 1000],
    loop_ptr: usize,
}

fn main() {
    let file_name = "test1.txt";

    let mut machine_state = MachineState {
        memory: [0; 1000],
        mem_ptr: 0,
        program: ['\0'; 1001],
        program_ctr: 0,
        loops: [0; 1000],
        loop_ptr: 0,
    };

    type ValueFunction = fn(&mut MachineState) -> bool;

    let mut char_map: HashMap<char, ValueFunction>= HashMap::new();
    char_map.insert('>', inc_ptr);
    char_map.insert('<', dec_ptr);
    char_map.insert('+', inc_loc);
    char_map.insert('-', dec_loc);
    char_map.insert('[', loop_begin);
    char_map.insert(']', loop_end);
    char_map.insert('.', put_char);

    let contents = fs::read_to_string(file_name)
        .expect("Could not read file");

    let in_str = contents
        .split("\n")
        .collect::<Vec<&str>>();

    let joined =  in_str.join("");

    let in_chars = joined.chars();

    // copy first 1000 chars from in_chars to machine_state.program
    for (i, c) in in_chars.enumerate() {
        if i > 999 {
            break;
        }
        machine_state.program[i] = c;
    }
    machine_state.program[1000] = '\0';

    while machine_state.program[machine_state.program_ctr] != '\0' {
        let c = machine_state.program[machine_state.program_ctr];
        if let Some(func) = char_map.get(&c) {
            if func(&mut machine_state) {
                machine_state.program_ctr+=1;
            }
        } 
    }
}

fn inc_ptr(state: &mut MachineState) -> bool {
    if state.mem_ptr < 999 {
        state.mem_ptr+=1;
    } else {
        state.mem_ptr = 0;
    }

    return true;
}

fn dec_ptr(state: &mut MachineState) -> bool {
    if state.mem_ptr > 0 {
        state.mem_ptr-=1;
    } else {
        state.mem_ptr = 1000;
    }

    return true;
} 

fn inc_loc(state: &mut MachineState) -> bool {
    state.memory[state.mem_ptr]+=1;
    return true;
}

fn dec_loc(state: &mut MachineState) -> bool {
    state.memory[state.mem_ptr]-=1;
    return true;
}

fn loop_begin(state: &mut MachineState) -> bool {
    state.program_ctr+=1;
    state.loops[state.loop_ptr] = state.program_ctr;
    state.loop_ptr+=1;

    return false;
}

fn loop_end(state: &mut MachineState) -> bool {
    if state.memory[state.mem_ptr] > 0 {
        state.program_ctr = state.loops[state.loop_ptr-1];
        return false;
    }
    state.loop_ptr-=1;

    return true;
}

fn put_char(state: &mut MachineState) -> bool {
    if let Some(c) = char::from_u32(state.memory[state.mem_ptr] as u32) {
        print!("{}", c);
    }

    return true;
}
