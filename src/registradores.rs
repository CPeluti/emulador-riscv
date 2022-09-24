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
    pub fn get_reg(&self, bin: &Option<&String>) -> i32 {
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
            _ => panic!("omegalul")
        }
    }

    pub fn get_pc(&self) -> &i32 {
        return &self.pc
    }
}
