use crate::bf_config;

type BfMemMap = [usize; bf_config::MEMORY_SIZE];

pub struct MachineState {
    pub memory: BfMemMap,
    pub mem_ptr: usize,
    pub program: [char; 1001],
    pub program_ctr: usize,
    pub loops: [usize; bf_config::MEMORY_SIZE],
    pub loop_ptr: usize,
}