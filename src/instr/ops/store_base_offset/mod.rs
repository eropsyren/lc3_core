use super::utils;
use crate::mem::Memory;

pub fn str(instr: u16, mem: &mut Memory) {
    let sr = (instr >> 9) & 0x7;
    let base_r = (instr >> 6) & 0x7;
    let offset = utils::sign_extend(instr & 0x3F, 6);

    let mem_location = mem.read_reg(base_r);
    let val = mem.read_reg(sr);

    mem.write(mem_location + offset, val);
}

#[cfg(test)]
mod tests {
    use super::str;
    use crate::mem::Memory;

    #[test]
    fn test_str() {
        let mut mem = Memory::new();
        let instr = 0b_0111_001_010_000010;
        mem.write_reg(1, 255);
        mem.write_reg(2, 10);
        str(instr, &mut mem);

        assert_eq!(255, mem.read(10 + 2));
    }
}
