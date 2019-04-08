use super::utils;
use crate::mem::Memory;

pub fn ldr(instr: u16, mem: &mut Memory) {
    let dr = (instr >> 9) & 0x7;
    let base_r = (instr >> 6) & 0x7;
    let offset = utils::sign_extend(instr & 0x3F, 6);

    let mem_location = mem.read_reg(base_r);
    let val = mem.read(mem_location + offset);

    mem.write_reg(dr, val);

    mem.update_flags(dr);
}

#[cfg(test)]
mod tests {
    use super::ldr;
    use crate::mem::Memory;
    use crate::mem::COND_POS;

    #[test]
    fn test_ldr() {
        let mut mem = Memory::new();
        let instr = 0b_0110_001_010_000001;
        mem.write_reg(2, 1);
        mem.write(2, 255);
        ldr(instr, &mut mem);

        assert_eq!(255, mem.read_reg(1));
        assert_eq!(COND_POS, mem.read_cond());
    }
}
