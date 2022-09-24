// Tipo R
pub fn add(rs1: i32, rs2: i32, rd: &mut i32) {
    *rd = rs1 + rs2;
}

pub fn sub(rs1: i32, rs2: i32, rd: &mut i32) {
    *rd = rs1 + rs2;
}

pub fn xor(rs1: i32, rs2: i32, rd: &mut i32) {
    *rd = rs1 ^ rs2;
}

pub fn or(rs1: i32, rs2: i32, rd: &mut i32) {
    *rd = rs1 | rs2;
}

pub fn and(rs1: i32, rs2: i32, rd: &mut i32) {
    *rd = rs1 & rs2;
}

pub fn sll(rs1: i32, rs2: i32, rd: &mut i32) {
    *rd = rs1 << rs2;
}

pub fn srl(rs1: i32, rs2: i32, rd: &mut i32) {
    *rd = rs1 >> rs2;
}

pub fn sra(rs1: i32, rs2: i32, rd: &mut i32) {
    *rd = if rs1 < rs2 {1} else {0};
}

pub fn slt(rs1: u32, rs2: u32, rd: &mut i32) {
    *rd = if rs1 < rs2 {1} else {0};
}

pub fn sltu(rs1: u32, rs2: u32, rd: &mut i32) {
    *rd = if rs1 < rs2 {1} else {0};
}


// Tipo I
pub fn addi(rs1: i32, imm: i32, rd: &mut i32) {
    *rd = rs1 + imm;
}

pub fn xori(rs1: i32, imm: i32, rd: &mut i32) {
    *rd = rs1 ^ imm;
}

pub fn ori(rs1: i32, imm: i32, rd: &mut i32) {
    *rd = rs1 | imm;
}

pub fn andi(rs1: i32, imm: i32, rd: &mut i32) {
    *rd = rs1 & imm;
}

pub fn slli(rs1: u32, imm: u32, rd: &mut i32) {
    *rd = (rs1 << (imm & 0b11111)) as i32;
}

pub fn srli(rs1: u32, imm: u32, rd: &mut i32) {
    *rd = (rs1 >> (imm & 0b11111)) as i32;
}

pub fn srai(rs1: i32, imm: i32, rd: &mut i32) {
    *rd = rs1 >> (imm & 0b11111);
}

pub fn slti(rs1: i32, imm: i32, rd: &mut i32) {
    *rd = if rs1 < imm {1} else {0};
}

pub fn sltiu(rs1: u32, imm: u32, rd: &mut i32) {
    *rd = if rs1 < imm {1} else {0};
}

pub fn lb(rs1: i32, imm: i32, rd: &mut i32, stack: Vec<i32>) {
    *rd = stack[(rs1 + imm) as usize] & 0b11111111
}

pub fn lh(rs1: i32, imm: i32, rd: &mut i32, stack: Vec<i32>) {
    *rd = stack[(rs1 + imm) as usize] & 0b1111111111111111
}

pub fn lw(rs1: i32, imm: i32, rd: &mut i32, stack: Vec<i32>) {
    *rd = stack[(rs1 + imm) as usize]
}

pub fn lbu(rs1: u32, imm: u32, rd: &mut i32, stack: Vec<i32>) {
    *rd = stack[(rs1 + imm) as usize] & 0b11111111
}

pub fn lhu(rs1: u32, imm: u32, rd: &mut i32, stack: Vec<i32>) {
    *rd = stack[(rs1 + imm) as usize] & 0b1111111111111111
}

pub fn jalr(rs1: i32, imm: i32, rd: &mut i32, pc: &mut i32) {
    *rd = *pc + 1;
    *pc = rs1 + imm;
}

pub fn ecall(rs1: i32, imm: i32, rd: &mut i32, pc: &mut i32) {
    // TODO
}

pub fn ebreak(rs1: i32, imm: i32, rd: &mut i32, pc: &mut i32) {
    // TODO
}


// Tipo S
pub fn sb(rs1: i32, rs2: i32, imm: i32, stack: &mut [i32]) {
    stack[(rs1 + imm) as usize] = (stack[(rs1 + imm) as usize] & -256) |
                        (rs2 & 0b11111111)
}

pub fn sh(rs1: i32, rs2: i32, imm: i32, stack: &mut [i32]) {
    stack[(rs1 + imm) as usize] = (stack[(rs1 + imm) as usize] & -65536) |
                        (rs2 & 0b1111111111111111)
}

pub fn sw(rs1: i32, rs2: i32, imm: i32, stack: &mut [i32]) {
    stack[(rs1 + imm) as usize] = rs2
}


// Tipo B
pub fn beq(rs1: i32, rs2: i32, imm: i32, pc: &mut i32) {
    *pc += if rs1 == rs2 {imm} else {0}
}

pub fn bne(rs1: i32, rs2: i32, imm: i32, pc: &mut i32) {
    *pc += if rs1 != rs2 {imm} else {0}
}

pub fn blt(rs1: i32, rs2: i32, imm: i32, pc: &mut i32) {
    *pc += if rs1 < rs2 {imm} else {0}
}

pub fn bge(rs1: i32, rs2: i32, imm: i32, pc: &mut i32) {
    *pc += if rs1 >= rs2 {imm} else {0}
}

pub fn bltu(rs1: u32, rs2: u32, imm: i32, pc: &mut i32) {
    *pc += if rs1 < rs2 {imm} else {0}
}

pub fn bgeu(rs1: u32, rs2: u32, imm: i32, pc: &mut i32) {
    *pc += if rs1 >= rs2 {imm} else {0}
}


// Tipo U
pub fn lui(imm: i32, rd: &mut i32) {
    *rd = imm << 12
}

pub fn auipc(imm: i32, pc: i32, rd: &mut i32) {
    *rd = pc + (imm << 12)
}


// Tipo J
pub fn jal(imm: i32, rd: &mut i32, pc: &mut i32) {
    *rd = *pc + 1;
    *pc = imm;
}
