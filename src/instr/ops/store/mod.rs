use super::utils;
use crate::mem::Memory;

pub fn st(instr: u16, mem: &mut Memory) {
    let sr = (instr >> 9) & 0x7;
    let pc_offset = utils::sign_extend(instr & 0x1FF, 9);

    let pc = mem.read_pc();
    let val = mem.read_reg(sr);

    mem.write(pc + pc_offset, val);
}

#[cfg(test)]
mod tests {
    use super::st;
    use crate::mem::Memory;

    #[test]
    fn test_st() {
        let mut mem = Memory::new();
        let instr = 0b_0011_001_000000100;
        mem.write_pc(1);
        mem.write_reg(1, 11);
        st(instr, &mut mem);

        assert_eq!(11, mem.read(1 + 4));
    }
}
