mod mem;
mod instr;

use mem::Memory;

pub struct LC3 {
    memory: Memory,
}