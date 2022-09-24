pub struct Registradores {
    let zero = 0;
    let mut pc = 0;
    let mut ra = 0;
    let mut sp = 0;
    let mut gp = 0;
    let mut tp = 0;
    let mut t0 = 0;
    let mut t1 = 0;
    let mut t2 = 0;
    let mut t3 = 0;
    let mut t4 = 0;
    let mut t5 = 0;
    let mut t6 = 0;
    let mut s0 = 0;
    let mut s1 = 0;
    let mut s2 = 0;
    let mut s3 = 0;
    let mut s4 = 0;
    let mut s5 = 0;
    let mut s6 = 0;
    let mut s7 = 0;
    let mut s8 = 0;
    let mut s9 = 0;
    let mut s10 = 0;
    let mut s11 = 0;
    let mut a0 = 0;
    let mut a1 = 0;
    let mut a2 = 0;
    let mut a3 = 0;
    let mut a4 = 0;
    let mut a5 = 0;
    let mut a6 = 0;
    let mut a7 = 0;
}

impl Registradores {
    fn get_reg(self, bin: String) {
        match bin {
            "00000" => return &self.zero
            "00001" => return &self.ra
            "00010" => return &self.sp
            "00011" => return &self.gp
            "00100" => return &self.tp
            "00101" => return &self.t0
            "00110" => return &self.t1
            "00111" => return &self.t2
            "01000" => return &self.s0
            "01001" => return &self.s1
            "01010" => return &self.a0
            "01011" => return &self.a1
            "01100" => return &self.a2
            "01101" => return &self.a3
            "01110" => return &self.a4
            "01111" => return &self.a5
            "10000" => return &self.a6
            "10001" => return &self.a7
            "10010" => return &self.s2
            "10011" => return &self.s3
            "10100" => return &self.s4
            "10101" => return &self.s5
            "10110" => return &self.s6
            "10111" => return &self.s7
            "11000" => return &self.s8
            "11001" => return &self.s9
            "11010" => return &self.s10
            "11011" => return &self.s11
            "11100" => return &self.t3
            "11101" => return &self.t4
            "11110" => return &self.t5
            "11111" => return &self.t6
        }
    }

    fn get_pc(self) {
        return &self.pc
    }
}
