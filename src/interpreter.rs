use std::borrow::Borrow;
use std::fs;
use regex::Regex;
use crate::instructions;
use crate::parser::Instruction;
use crate::registradores::Registradores;

pub fn interpret(instruction_list: Vec<Instruction>) -> () {
    //pilha
    let mut stack: Vec<Instruction> = instruction_list;
    let mut stack2: Vec<i32>;
    let regs: Registradores = Registradores { zero: (0), pc: (0), ra: (0), sp: (0), gp: (0), tp: (0), t0: (0), t1: (0), t2: (0), t3: (0), t4: (0), t5: (0), t6: (0), s0: (0), s1: (0), s2: (0), s3: (0), s4: (0), s5: (0), s6: (0), s7: (0), s8: (0), s9: (0), s10: (0), s11: (0), a0: (0), a1: (0), a2: (0), a3: (0), a4: (0), a5: (0), a6: (0), a7: (0) };

    loop {
        let instruction: Result<&Instruction, &str> = stack.get(*regs.get_pc() as usize).ok_or("no instruction");
        let inst = match instruction {
            Ok(inst) => inst,
            Err(_) => panic!("Instruction not found")
        };
        let opcode = &inst.opcode;
        let f3 = &inst.funct3;
        let f7 = &inst.funct7;

        // Breve refatorada aqui, código do Ligoski infelizmente não funcionava
        // let Instruction { opcode: op, funct3: f3, funct7: f7, .. } = instruction;    (antes era isso)
        match opcode.as_ref().map(|x| &**x) {

            // Tipo I loads
            Some("0000011") => {
                //let mut rs1, imm, rd: i32, i32, i32;
                let rs1 = &regs.get_reg(&inst.rs1.as_ref());
                let mut rd = &regs.get_reg(&inst.rd.as_ref());
                let imm = inst.imm.as_ref().unwrap().parse::<i32>().unwrap();
                //rd = regs.get_reg(inst.rd);
                //imm = inst.imm.to_num //TODO
                match f3.as_ref().map(|x| &**x) {
                    // lb
                    Some("000") => {
                        instructions::lb(*rs1, imm, &mut rd, stack2)
                    }
                    //lh
                    Some("001") => {}
                    //lw
                    Some("010") => {}
                    //lbu
                    Some("100") => {}
                    //lhu
                    Some("101") => {}
                    _ => {
                        panic!("Invalid funct3 value")
                    }
                }
            }
            // Tipo I
            Some("0000111") | Some("0010011") => {
                match f3.as_ref().map(|x| &**x) {
                    // addi
                    Some("000") => {}
                    //slli
                    Some("001") => {}
                    //slti
                    Some("010") => {}
                    //sltiu
                    Some("011") => {}
                    //xori
                    Some("100") => {}
                    //tipo I shift
                    Some("101") => {
                        match f7.as_ref().map(|x| &**x) {
                            //srli
                            Some("0000000") => {}
                            //srai
                            Some("0100000") => {}
                            _ => {}
                        }
                    }
                    //ori
                    Some("110") => {}
                    //andi
                    Some("111") => {}
                    _ => {
                        panic!("Invalid funct3 value")
                    }
                }
            }
            // tipo U - auipc
            Some("0010111") => {}
            // tipo U - lui
            Some("0110111") => {}
            // tipo S
            Some("0100111") | Some("0100011") => {
                match f3.as_ref().map(|x| &**x) {
                    //sb
                    Some("000") => {}
                    //sh
                    Some("001") => {}
                    //sw
                    Some("010") => {}
                    _ => {
                        panic!("Invalid funct3 value")
                    }
                }
            }
            //jalr
            Some("1100111") => {}
            //jal
            Some("1101111") => {}

            // tipo R
            Some("0110011") => {
                match f7.as_ref().map(|x| &**x) {
                    Some("0000000") => {
                        match f3.as_ref().map(|x| &**x) {
                            //add
                            Some("000") => {
                                // instructions::add(rs1, rs2, rd);
                            }
                            //sll
                            Some("001") => {}
                            //slt
                            Some("010") => {}
                            //sltu
                            Some("011") => {}
                            //xor
                            Some("100") => {}
                            //srl
                            Some("101") => {}
                            //or
                            Some("110") => {}
                            //and
                            Some("111") => {}
                            _ => {
                                panic!("Invalid funct7 value")
                            }
                        }
                    }
                    Some("0100000") => {
                        match f3.as_ref().map(|x| &**x) {
                            //sub
                            Some("000") => {}
                            //sra
                            Some("101") => {}
                            _ => {
                                panic!("Invalid funct3 value")
                            }
                        }
                    }
                    Some("0000001") => {
                        match f3.as_ref().map(|x| &**x) {
                            //mul
                            Some("000") => {}
                            //mulh
                            Some("001") => {}
                            //mulhsu
                            Some("010") => {}
                            //mulhu
                            Some("011") => {}
                            //div
                            Some("100") => {}
                            //divu
                            Some("101") => {}
                            //rem
                            Some("110") => {}
                            //remu
                            Some("111") => {}
                            //mulh
                            Some("101") => {}
                            _ => {
                                panic!("Invalid funct3 value")
                            }
                        }
                    }
                    _ => {}
                }
            }

            //Tipo I
            Some("1110011") => {
                match f3.as_ref().map(|x| &**x) {
                    //ecall / uret
                    Some("000") => {}
                    //csrrw
                    Some("001") => {}
                    //csrrs
                    Some("010") => {}
                    //csrrc
                    Some("011") => {}
                    //csrrwi
                    Some("101") => {}
                    //csrrsi
                    Some("110") => {}
                    //csrrci
                    Some("111") => {}
                    _ => {
                        panic!("Invalid funct3 value")
                    }
                }
            }
            // tipo B
            Some("1100011") => {
                match f3.as_ref().map(|x| &**x) {
                    //beq
                    Some("000") => {}
                    //bne
                    Some("001") => {}
                    //blt
                    Some("100") => {}
                    //bge
                    Some("101") => {}
                    //bltu
                    Some("110") => {}
                    //bgeu
                    Some("111") => {}
                    _ => {
                        panic!("Invalid funct3 value")
                    }
                }
            }
            _ => {
                panic!("Invalid opcode");
            }
        }
    }
}
