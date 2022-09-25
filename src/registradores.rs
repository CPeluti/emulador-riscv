use std::string;

pub struct Registradores {
    pub zero: i32,
    pub pc: i32,
    pub ra: i32,
    pub sp: i32,
    pub gp: i32,
    pub tp: i32,
    pub t0: i32,
    pub t1: i32,
    pub t2: i32,
    pub t3: i32,
    pub t4: i32,
    pub t5: i32,
    pub t6: i32,
    pub s0: i32,
    pub s1: i32,
    pub s2: i32,
    pub s3: i32,
    pub s4: i32,
    pub s5: i32,
    pub s6: i32,
    pub s7: i32,
    pub s8: i32,
    pub s9: i32,
    pub s10: i32,
    pub s11: i32,
    pub a0: i32,
    pub a1: i32,    
    pub a2: i32,
    pub a3: i32,
    pub a4: i32,
    pub a5: i32,
    pub a6: i32,
    pub a7: i32,
}

impl Registradores {
    pub fn get_reg(&mut self, bin: &Option<&String>) -> i32 {
        let register_reference = match bin {
            Some(string) => {
                string
            }
            None => {
                panic!("sexy ass")
            }
        };
        let register = &**register_reference;
        let r = &register[..];

        match r {
            "00000" => return self.zero,
            "00001" => return self.ra,
            "00010" => return self.sp,
            "00011" => return self.gp,
            "00100" => return self.tp,
            "00101" => return self.t0,
            "00110" => return self.t1,
            "00111" => return self.t2,
            "01000" => return self.s0,
            "01001" => return self.s1,
            "01010" => return self.a0,
            "01011" => return self.a1,
            "01100" => return self.a2,
            "01101" => return self.a3,
            "01110" => return self.a4,
            "01111" => return self.a5,
            "10000" => return self.a6,
            "10001" => return self.a7,
            "10010" => return self.s2,
            "10011" => return self.s3,
            "10100" => return self.s4,
            "10101" => return self.s5,
            "10110" => return self.s6,
            "10111" => return self.s7,
            "11000" => return self.s8,
            "11001" => return self.s9,
            "11010" => return self.s10,
            "11011" => return self.s11,
            "11100" => return self.t3,
            "11101" => return self.t4,
            "11110" => return self.t5,
            "11111" => return self.t6,
            _ => panic!("Register not found: {}, original: {:?}", r, bin)
        }
    }

    pub fn set_reg(&mut self, target_register: &i32, target_value: i32) -> () {
        //let register_reference = match target_register {
        //    Some(string) => {
        //        string
        //    }
        //    None => {
        //        panic!("sexy ass")
        //    }
        //};
        //let register = &**register_reference;
        //let r = &register[..];

        match target_register {
            00000 => return,
            00001 => self.ra = target_value,
            00010 => self.sp = target_value,
            00011 => self.gp = target_value,
            00100 => self.tp = target_value,
            00101 => self.t0 = target_value,
            00110 => self.t1 = target_value,
            00111 => self.t2 = target_value,
            01000 => self.s0 = target_value,
            01001 => self.s1 = target_value,
            01010 => self.a0 = target_value,
            01011 => self.a1 = target_value,
            01100 => self.a2 = target_value,
            01101 => self.a3 = target_value,
            01110 => self.a4 = target_value,
            01111 => self.a5 = target_value,
            10000 => self.a6 = target_value,
            10001 => self.a7 = target_value,
            10010 => self.s2 = target_value,
            10011 => self.s3 = target_value,
            10100 => self.s4 = target_value,
            10101 => self.s5 = target_value,
            10110 => self.s6 = target_value,
            10111 => self.s7 = target_value,
            11000 => self.s8 = target_value,
            11001 => self.s9 = target_value,
            11010 => self.s10 = target_value,
            11011 => self.s11 = target_value,
            11100 => self.t3 = target_value,
            11101 => self.t4 = target_value,
            11110 => self.t5 = target_value,
            11111 => self.t6 = target_value,
            _ => panic!("omegalul")
        }
    }

    pub fn get_pc(&mut self) -> &i32 {
        return &self.pc
    }

    pub fn set_pc(&mut self, target_value: i32) -> () {
        self.pc = target_value;
    }
}
