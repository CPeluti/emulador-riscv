use std::borrow::Borrow;
use std::fs;
use regex::Regex;
use crate::instructions;
use crate::parser::Instruction;
use crate::registradores::Registradores;

pub fn interpret(instruction_list: Vec<Instruction>) -> () {
    //pilha
    let mut stack_inst: Vec<Instruction> = instruction_list;
    let mut stack = vec![0i32; 1024];
    let mut regs: Registradores = Registradores { zero: (0), pc: (0), ra: (0), sp: (0), gp: (0), tp: (0), t0: (0), t1: (0), t2: (0), t3: (0), t4: (0), t5: (0), t6: (0), s0: (0), s1: (0), s2: (0), s3: (0), s4: (0), s5: (0), s6: (0), s7: (0), s8: (0), s9: (0), s10: (0), s11: (0), a0: (0), a1: (0), a2: (0), a3: (0), a4: (0), a5: (0), a6: (0), a7: (0) };

    loop {
        let instruction: Result<&Instruction, &str> = stack_inst.get(*regs.get_pc() as usize).ok_or("no instruction");
        let inst = match instruction {
            Ok(inst) => inst,
            Err(_) => panic!("Instruction not found")
        };
        let opcode = &inst.opcode;
        let f3 = &inst.funct3;
        let f7 = &inst.funct7;

        // Breve refatorada aqui, código do Ligoski infelizmente não funcionava
        // let Instruction { opcode: op, funct3: f3, funct7: f7, .. } = instruction;    (antes era isso)
        let mut x: i32 = 0;
        let mut target_pc: i32 = 0;
        match opcode.as_ref().map(|x| &**x) {

            // Tipo I loads
            Some("0000011") => {
                let rs1 = &regs.get_reg(&inst.rs1.as_ref());
                let imm = i32::from_str_radix(inst.imm.as_ref().unwrap(), 2).unwrap();
                let mut rd = &regs.get_reg(&inst.rd.as_ref());
                match f3.as_ref().map(|x| &**x) {
                    // lb
                    Some("000") => {
                        instructions::lb(*rs1, imm, &mut x, &stack);
                        regs.set_reg(rd, x);
                    }
                    //lh
                    Some("001") => {
                        instructions::lh(*rs1, imm, &mut x, &stack);
                        regs.set_reg(rd, x);
                    }
                    //lw
                    Some("010") => {
                        instructions::lw(*rs1, imm, &mut x, &stack);
                        regs.set_reg(rd, x);
                    }
                    //lbu
                    Some("100") => {
                        instructions::lbu(*rs1 as u32, imm as u32, &mut x, &stack);
                        regs.set_reg(rd, x);
                    }
                    //lhu
                    Some("101") => {
                        instructions::lhu(*rs1 as u32, imm as u32, &mut x, &stack);
                        regs.set_reg(rd, x);
                    }
                    _ => {
                        panic!("Invalid funct3 value");
                    }
                }
            }
            // Tipo I
            Some("0000111") | Some("0010011") => {
                let rs1 = &regs.get_reg(&inst.rs1.as_ref());
                let imm = inst.imm.as_ref().unwrap().parse::<i32>().unwrap();
                let mut rd = &regs.get_reg(&inst.rd.as_ref());
                match f3.as_ref().map(|x| &**x) {
                    // addi
                    Some("000") => {
                        instructions::addi(*rs1, imm, &mut x);
                        regs.set_reg(rd, x);
                    }
                    //slli
                    Some("001") => {
                        instructions::slli(*rs1 as u32, imm as u32, &mut x);
                        regs.set_reg(rd, x);
                    }
                    //slti
                    Some("010") => {
                        instructions::slti(*rs1, imm, &mut x);
                        regs.set_reg(rd, x);
                    }
                    //sltiu
                    Some("011") => {
                        instructions::sltiu(*rs1 as  u32, imm as u32, &mut x);
                        regs.set_reg(rd, x);
                    }
                    //xori
                    Some("100") => {
                        instructions::xori(*rs1, imm, &mut x);
                        regs.set_reg(rd, x);
                    }
                    //tipo I shift
                    Some("101") => {
                        match f7.as_ref().map(|x| &**x) {
                            //srli
                            Some("0000000") => {
                                instructions::srli(*rs1 as u32, imm as u32, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //srai
                            Some("0100000") => {
                                instructions::slti(*rs1, imm, &mut x);
                                regs.set_reg(rd, x);
                            }
                            _ => {}
                        }
                    }
                    //ori
                    Some("110") => {
                        instructions::ori(*rs1, imm, &mut x);
                        regs.set_reg(rd, x);
                    }
                    //andi
                    Some("111") => {
                        instructions::andi(*rs1, imm, &mut x);
                        regs.set_reg(rd, x);
                    }
                    _ => {
                        panic!("Invalid funct3 value");
                    }
                }
            }
            // tipo U - auipc
            Some("0010111") => {
                let imm = inst.imm.as_ref().unwrap().parse::<i32>().unwrap();
                let rd = &regs.get_reg(&inst.rd.as_ref());
                instructions::auipc(imm, *regs.get_pc(), &mut x);
                regs.set_reg(rd, x);
            }
            // tipo U - lui
            Some("0110111") => {
                let imm = inst.imm.as_ref().unwrap().parse::<i32>().unwrap();
                let rd = &regs.get_reg(&inst.rd.as_ref());
                instructions::lui(imm, &mut x);
                regs.set_reg(rd, x);
            }
            // tipo S
            Some("0100111") | Some("0100011") => {
                let rs1 = &regs.get_reg(&inst.rs1.as_ref());
                let rs2 = &regs.get_reg(&inst.rs2.as_ref());
                let imm = inst.imm.as_ref().unwrap().parse::<i32>().unwrap();
                match f3.as_ref().map(|x| &**x) {
                    //sb
                    Some("000") => {
                        instructions::sb(*rs1, *rs2, imm, &mut stack);
                    }
                    //sh
                    Some("001") => {
                        instructions::sb(*rs1, *rs2, imm, &mut stack);
                    }
                    //sw
                    Some("010") => {
                        instructions::sb(*rs1, *rs2, imm, &mut stack);
                    }
                    _ => {
                        panic!("Invalid funct3 value");
                    }
                }
            }
            //jalr
            Some("1100111") => {
                let rs1 = &regs.get_reg(&inst.rs1.as_ref());
                let imm = inst.imm.as_ref().unwrap().parse::<i32>().unwrap();
                let mut rd = &regs.get_reg(&inst.rd.as_ref());
                instructions::jalr(*rs1, imm, &mut x, &mut target_pc);

                regs.set_reg(rd, x);
                regs.set_pc(target_pc);
            }
            //jal
            Some("1101111") => {
                let imm = inst.imm.as_ref().unwrap().parse::<i32>().unwrap();
                let mut rd = &regs.get_reg(&inst.rd.as_ref());
                instructions::jal(imm, &mut x, &mut target_pc);

                regs.set_reg(rd, x);
                regs.set_pc(target_pc);
            }

            // tipo R
            Some("0110011") => {
                let rs1 = &regs.get_reg(&inst.rs1.as_ref());
                let rs2 = &regs.get_reg(&inst.rs2.as_ref());
                let mut rd = &regs.get_reg(&inst.rd.as_ref());
                match f7.as_ref().map(|x| &**x) {
                    Some("0000000") => {
                        match f3.as_ref().map(|x| &**x) {
                            //add
                            Some("000") => {
                                instructions::add(*rs1, *rs2, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //sll
                            Some("001") => {
                                instructions::sll(*rs1 as u32, *rs2 as u32, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //slt
                            Some("010") => {
                                instructions::slt(*rs1, *rs2, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //sltu
                            Some("011") => {
                                instructions::sltu(*rs1 as u32, *rs2 as u32, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //xor
                            Some("100") => {
                                instructions::xor(*rs1, *rs2, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //srl
                            Some("101") => {
                                instructions::srl(*rs1 as u32, *rs2 as u32, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //or
                            Some("110") => {
                                instructions::or(*rs1, *rs2, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //and
                            Some("111") => {
                                instructions::and(*rs1, *rs2, &mut x);
                                regs.set_reg(rd, x);
                            }
                            _ => {
                                panic!("Invalid funct7 value");
                            }
                        }
                    }
                    Some("0100000") => {
                        match f3.as_ref().map(|x| &**x) {
                            //sub
                            Some("000") => {
                                instructions::sub(*rs1, *rs2, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //sra
                            Some("101") => {
                                instructions::sra(*rs1, *rs2, &mut x);
                                regs.set_reg(rd, x);
                            }
                            _ => {
                                panic!("Invalid funct3 value");
                            }
                        }
                    }
                    Some("0000001") => {
                        match f3.as_ref().map(|x| &**x) {
                            //mul
                            Some("000") => {
                                instructions::mul(*rs1, *rs2, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //mulh
                            Some("001") => {
                                instructions::mulh(*rs1 as i64, *rs2 as i64, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //mulhsu
                            Some("010") => {
                                instructions::mulhsu(*rs1 as i64, *rs2 as u64, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //mulhu
                            Some("011") => {
                                instructions::mulhu(*rs1 as u64, *rs2 as u64, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //div
                            Some("100") => {
                                instructions::div(*rs1, *rs2, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //divu
                            Some("101") => {
                                instructions::divu(*rs1 as u32, *rs2 as u32, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //rem
                            Some("110") => {
                                instructions::rem(*rs1, *rs2, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //remu
                            Some("111") => {
                                instructions::remu(*rs1 as u32, *rs2 as u32, &mut x);
                                regs.set_reg(rd, x);
                            }
                            //mulh
                            Some("101") => {
                                instructions::mulh(*rs1 as i64, *rs2 as i64, &mut x);
                                regs.set_reg(rd, x);
                            }
                            _ => {
                                panic!("Invalid funct3 value");
                            }
                        }
                    }
                    _ => {}
                }
            }

            //Tipo I
            Some("1110011") => {
                let rs1 = &regs.get_reg(&inst.rs1.as_ref());
                let imm = inst.imm.as_ref().unwrap().parse::<i32>().unwrap();
                let mut rd = &regs.get_reg(&inst.rd.as_ref());
                match f3.as_ref().map(|x| &**x) {
                    //ecall / uret
                    Some("000") => {
                        instructions::ecall(&mut x, *rs1, &mut target_pc);

                        regs.set_reg(rd, x);
                        regs.set_pc(target_pc);
                    }
                    /*
                    //csrrw
                    Some("001") => {
                    }
                    //csrrs
                    Some("010") => {
                    }
                    //csrrc
                    Some("011") => {
                    }
                    //csrrwi
                    Some("101") => {
                    }
                    //csrrsi
                    Some("110") => {
                    }
                    //csrrci
                    Some("111") => {}
                    */
                    _ => {
                        panic!("Invalid funct3 value")
                    }
                }
            }
            // tipo B
            Some("1100011") => {
                let rs1 = &regs.get_reg(&inst.rs1.as_ref());
                let rs2 = &regs.get_reg(&inst.rs2.as_ref());
                let imm = inst.imm.as_ref().unwrap().parse::<i32>().unwrap();
                match f3.as_ref().map(|x| &**x) {
                    //beq
                    Some("000") => {
                        instructions::beq(*rs1, *rs2, imm, &mut target_pc);
                        regs.set_pc(target_pc);
                    }
                    //bne
                    Some("001") => {
                        instructions::bne(*rs1, *rs2, imm, &mut target_pc);
                        regs.set_pc(target_pc);
                    }
                    //blt
                    Some("100") => {
                        instructions::blt(*rs1, *rs2, imm, &mut target_pc);
                        regs.set_pc(target_pc);
                    }
                    //bge
                    Some("101") => {
                        instructions::bge(*rs1, *rs2, imm, &mut target_pc);
                        regs.set_pc(target_pc);
                    }
                    //bltu
                    Some("110") => {
                        instructions::bltu(*rs1 as u32, *rs2 as u32, imm, &mut target_pc);
                        regs.set_pc(target_pc);
                    }
                    //bgeu
                    Some("111") => {
                        instructions::bgeu(*rs1 as u32, *rs2 as u32, imm, &mut target_pc);
                        regs.set_pc(target_pc);
                    }
                    _ => {
                        panic!("Invalid funct3 value");
                    }
                }
            }
            _ => {
                panic!("Invalid opcode");
            }
        }
    }
}
