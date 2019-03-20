use super::utils;
use crate::mem::Memory;

pub fn sti(instr: u16, mem: &mut Memory) {
    let sr = (instr >> 9) & 0x7;
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let pc = mem.read_pc();
    let mem_location = mem.read(pc + pc_offset);
    let val = mem.read_reg(sr);

    mem.write(mem_location, val);
}

#[cfg(test)]
mod tests {
    use super::sti;
    use crate::mem::Memory;

    #[test]
    fn test_sti() {
        let mut mem = Memory::new();
        let instr = 0b_1011_001_000000001;
        mem.write_reg(1, 255);
        mem.write_pc(99);
        mem.write(99 + 1, 121);
        sti(instr, &mut mem);

        assert_eq!(255, mem.read(121));
    }
}
