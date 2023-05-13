use crate::bf_config;

type BfMemMap = [usize; bf_config::MEMORY_SIZE];

pub struct Memory {
    memory: BfMemMap,
}

impl Memory {

    pub fn get_memory_at_address(&mut self, addr: usize) -> usize {
        return self.memory[addr];
    }

    pub fn inc_memory_at_address(&mut self, addr: usize) {
        self.memory[addr]+=1;
    }

    pub fn dec_memory_at_address(&mut self, addr: usize) {
        self.memory[addr]-=1;
    }
}

pub fn initialize_memory() -> Memory {
    return Memory {
        memory: [0; bf_config::MEMORY_SIZE], 
    };
}