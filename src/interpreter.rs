use std::borrow::Borrow;
use std::fs;
use regex::Regex;
use crate::parser::Instruction;

pub fn interpret(instruction_list: Vec<Instruction>) -> () {
    //pilha
    let mut stack: Vec<Instruction> = instruction_list;

    // registradores
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


    loop {
        let intruction: Option<&Instruction> = stack.get(pc);
        let Instruction { opcode: op, funct3: f3, funct7: f7, .. } = intruction;
        match op.as_ref().map(|x| &**x) {

            // Tipo I loads
            Some("0000011") => {
                match f3.as_ref().map(|x| &**x) {
                    // lb
                    Some("000") => {}
                    //lh
                    Some("001") => {}
                    //lw
                    Some("010") => {}
                    //lbu
                    Some("100") => {}
                    //lhu
                    Some("101") => {}
                    _ => {}
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
                    _ => {}
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
                    _ => {}
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
                            Some("000") => {}
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
                            _ => {}
                        }
                    }
                    Some("0100000") => {
                        match f3.as_ref().map(|x| &**x) {
                            //sub
                            Some("000") => {}
                            //sra
                            Some("101") => {}
                            _ => {}
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
                            _ => {}
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
                    _ => {}
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
                    _ => {}
                }
            }
            _ => {
                println!("sexo2");
            }
        }
    }
}
