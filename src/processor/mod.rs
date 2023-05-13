use crate::bf_config;
use crate::bf_instructions;
use crate::display;
use crate::memory;
use crate::rom;

pub struct Processor {
    display: display::Display,
    memory: memory::Memory,
    rom: rom::Rom,
    mem_ptr: usize,
    program_ctr: usize,
    loops: [usize; bf_config::MEMORY_SIZE],
    loop_ptr: usize,
    skip_next_pc_step: bool,
}

impl Processor {

    pub fn start(&mut self) {

        let mut pc = self.get_program_counter();

        while self.rom.get_code_at(pc) != '\0' {
            let c = self.rom.get_code_at(pc);
            match c {
                bf_instructions::PTR_INC => self.inc_mem_ptr(),
                bf_instructions::PTR_DEC => self.dec_mem_ptr(),
                bf_instructions::LOC_INC => self.inc_mem_loc_val(),
                bf_instructions::LOC_DEC => self.dec_mem_loc_val(),
                bf_instructions::LOOP_BEG => self.loop_begin(),
                bf_instructions::LOOP_END => self.loop_end(),
                bf_instructions::PUT_CHAR => self.display_char(),
                _ => self.nop(),
            }

            if !self.should_skip_next_pc_step() {
                self.step_program_counter();
            }

            pc = self.get_program_counter();
        }
    }

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

    pub fn display_char(&mut self) {
        self.skip_next_pc_step = false;

        if let Some(c) = char::from_u32(self.memory.get_memory_at_address(self.mem_ptr) as u32) {
            self.display.output_char(c);
        }
    }

    pub fn nop(&mut self) { 
        self.skip_next_pc_step = false;
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

pub fn initialize_machine(mem: memory::Memory, rom: rom::Rom, display: display::Display) -> Processor {
    return Processor {
        display: display,
        memory: mem,
        rom: rom,
        mem_ptr: 0,
        program_ctr: 0,
        loops: [0; bf_config::MEMORY_SIZE],
        loop_ptr: 0,
        skip_next_pc_step: false,
    };
}