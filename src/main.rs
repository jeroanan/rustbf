use std::collections::HashMap;
use std::fs;

type BfMemMap = [i32; 1000];

struct MachineState {
    memory: BfMemMap,
    mem_ptr: usize,
    program: String,
    program_ctr: usize,
    loops: BfMemMap,
    loop_ptr: usize,
}

fn main() {
    let file_name = "test1.txt";

    let mut machine_state = MachineState {
        memory: [0; 1000],
        mem_ptr: 0,
        program: String::from(""),
        program_ctr: 0,
        loops: [0; 1000],
        loop_ptr: 0,
    };

    type ValueFunction = fn(&mut MachineState) -> ();

    let mut char_map: HashMap<char, ValueFunction>= HashMap::new();
    char_map.insert('>', inc_ptr);
    char_map.insert('<', dec_ptr);
    char_map.insert('+', |state| state.memory[state.mem_ptr]+=1);
    char_map.insert('-', |state| state.memory[state.mem_ptr]-=1);
    char_map.insert('[', loop_begin);

    let contents = fs::read_to_string(file_name)
        .expect("Could not read file");

    let in_chars = contents
        .split("\n")
        .collect::<Vec<&str>>()
        .join("");

    machine_state.program = in_chars.clone();

    for c in in_chars.chars() {
        if let Some(func) = char_map.get(&c) {
            func(&mut machine_state);
        }
    }

    println!("{:?}", machine_state.mem_ptr)

}

fn inc_ptr(state: &mut MachineState) {
    if state.mem_ptr < 999 {
        state.mem_ptr+=1;
    } else {
        state.mem_ptr = 0;
    }
}

fn dec_ptr(state: &mut MachineState) {
    if state.mem_ptr > 0 {
        state.mem_ptr-=1;
    } else {
        state.mem_ptr = 1000;
    }
} 

fn loop_begin(state: &mut MachineState) {
    state.loops[state.loop_ptr] = state.memory[state.mem_ptr+1];
    state.loop_ptr+=1;
}

/* 
fn loop_end(m: &mut BfMemMap, mr: &mut usize, l: &mut BfMemMap, lp: &mut usize) {
}
*/
