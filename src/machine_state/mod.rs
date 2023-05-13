const MEMORY_SIZE: usize = 1000;
type BfMemMap = [usize; MEMORY_SIZE];

pub struct MachineState {
    pub memory: BfMemMap,
    pub mem_ptr: usize,
    pub program: [char; 1001],
    pub program_ctr: usize,
    pub loops: [usize; 1000],
    pub loop_ptr: usize,
}