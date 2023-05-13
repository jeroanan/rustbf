use crate::bf_config;

type BfMemMap = [usize; bf_config::MEMORY_SIZE];

pub struct MachineState {
    pub memory: BfMemMap,
    mem_ptr: usize,
    pub program: [char; bf_config::MEMORY_SIZE+1],
    pub program_ctr: usize,
    loops: [usize; bf_config::MEMORY_SIZE],
    loop_ptr: usize,
    pub skip_next_pc_step: bool,
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
        self.memory[self.mem_ptr]+=1;
        self.skip_next_pc_step = false;
    }

    pub fn dec_mem_loc_val(&mut self) {
        self.memory[self.mem_ptr]-=1;
        self.skip_next_pc_step = false;
    }

    pub fn loop_begin(&mut self) {
        self.program_ctr+=1;
        self.loops[self.loop_ptr] = self.program_ctr;
        self.loop_ptr+=1;
        self.skip_next_pc_step = true;
    }

    pub fn loop_end(&mut self) {
        if self.memory[self.mem_ptr] > 0 {
            self.program_ctr = self.loops[self.loop_ptr-1];
            self.skip_next_pc_step = true;
            return;
        }
        self.loop_ptr-=1;
    
        self.skip_next_pc_step = false;
    }

    pub fn get_char_to_display(&mut self) -> char {
        self.skip_next_pc_step = false;

        if let Some(c) = char::from_u32(self.memory[self.mem_ptr] as u32) {
            return c;
        }
        return '\0';
    }

    pub fn step_program_counter(&mut self) {
        self.program_ctr+=1;
    }

}

pub fn initialize_machine() -> MachineState {
    return MachineState {
        memory: [0; bf_config::MEMORY_SIZE],
        mem_ptr: 0,
        program: ['\0'; bf_config::MEMORY_SIZE+1],
        program_ctr: 0,
        loops: [0; bf_config::MEMORY_SIZE],
        loop_ptr: 0,
        skip_next_pc_step: false,
    };
}