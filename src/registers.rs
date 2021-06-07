fn join_u8_to_u16(high: u8, low: u8) -> u16 {
    (high as u16) << 8 | (low as u16)
}

fn split_u16_to_u8(val: u16) -> (u8, u8) {
    (
        ((val & 0xFF00) >> 8) as u8,
        (val & 0x00FF) as u8,
    )
}

pub struct Registers {
    // http://bgb.bircd.org/pandocs.htm#cpuregistersandflags
    pub a: u8,
    pub flag: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub stack_pointer: u16,
    pub program_counter: u16,
    // Not listed: the 16-bit registers AF, BC, DE and HL, which formed by
    // combining registers a and flag, b and c, d and e, and h and l,
    // respectively
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            flag: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            stack_pointer: 0,
            program_counter: 0
        }
    }

    pub fn get_af(&self) -> u16 {
        join_u8_to_u16(self.a, self.flag)
    }

    pub fn set_af(&mut self, new_af: u16) {
        let (new_a, new_flag) = split_u16_to_u8(new_af);
        self.a = new_a;
        self.flag = new_flag;
    }

    pub fn get_bc(&self) -> u16 {
        join_u8_to_u16(self.b, self.c)
    }

    pub fn set_bc(&mut self, new_bc: u16) {
        let (new_b, new_c) = split_u16_to_u8(new_bc);
        self.b = new_b;
        self.c = new_c;
    }

    pub fn get_de(&self) -> u16 {
        join_u8_to_u16(self.d, self.e)
    }

    pub fn set_de(&mut self, new_de: u16) {
        let (new_d, new_e) = split_u16_to_u8(new_de);
        self.d = new_d;
        self.e = new_e;
    }

    pub fn get_hl(&self) -> u16 {
        join_u8_to_u16(self.h, self.l)
    }

    pub fn set_hl(&mut self, new_hl: u16)  {
        let (new_h, new_l) = split_u16_to_u8(new_hl);
        self.h = new_h;
        self.l = new_l;
    }
}
