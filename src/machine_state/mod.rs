use crate::bf_config;
use crate::memory;

pub struct MachineState {
    memory: memory::Memory,
    mem_ptr: usize,
    program_ctr: usize,
    loops: [usize; bf_config::MEMORY_SIZE],
    loop_ptr: usize,
    skip_next_pc_step: bool,
}

impl MachineState {

    pub fn inc_mem_ptr(&mut self) {
        if self.mem_ptr < bf_config::MEMORY_SIZE  {
            self.mem_ptr+=1;
        } else {
            self.mem_ptr = 0;
        }
        self.skip_next_pc_step = false;
    }

    pub fn dec_mem_ptr(&mut self) {
        if self.mem_ptr > 0 {
            self.mem_ptr-=1;
        } else {
            self.mem_ptr = bf_config::MEMORY_SIZE;
        }
        self.skip_next_pc_step = false;
    }

    pub fn inc_mem_loc_val(&mut self) {
        self.memory.inc_memory_at_address(self.mem_ptr);
        self.skip_next_pc_step = false;
    }

    pub fn dec_mem_loc_val(&mut self) {
        self.memory.dec_memory_at_address(self.mem_ptr);
        self.skip_next_pc_step = false;
    }

    pub fn loop_begin(&mut self) {
        self.program_ctr+=1;
        self.loops[self.loop_ptr] = self.program_ctr;
        self.loop_ptr+=1;
        self.skip_next_pc_step = true;
    }

    pub fn loop_end(&mut self) {
        if self.memory.get_memory_at_address(self.mem_ptr) > 0 { //self.memory[self.mem_ptr] > 0 {
            self.program_ctr = self.loops[self.loop_ptr-1];
            self.skip_next_pc_step = true;
            return;
        }
        self.loop_ptr-=1;
    
        self.skip_next_pc_step = false;
    }

    pub fn get_char_to_display(&mut self) -> char {
        self.skip_next_pc_step = false;

        if let Some(c) = char::from_u32(self.memory.get_memory_at_address(self.mem_ptr) as u32) {
            return c;
        }
        return '\0';
    }

    pub fn step_program_counter(&mut self) {
        self.program_ctr+=1;
    }

    pub fn get_program_counter(&mut self) -> usize {
        return self.program_ctr;
    }

    pub fn should_skip_next_pc_step(&mut self) -> bool {
        return self.skip_next_pc_step;
    }

}

pub fn initialize_machine(mem: memory::Memory) -> MachineState {
    return MachineState {
        memory: mem,
        mem_ptr: 0,
        program_ctr: 0,
        loops: [0; bf_config::MEMORY_SIZE],
        loop_ptr: 0,
        skip_next_pc_step: false,
    };
}