mod bf_config;
mod bf_instructions;
mod display;
mod memory;
mod processor;
mod rom;

fn main() {

    let mut rom = rom::initialize_rom();
    rom.read_program_file();

    let memory = memory::initialize_memory();
    let display = display::Display {};

    let mut proc = processor::initialize_machine(memory, rom, display);
    proc.start();
}
