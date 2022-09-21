use std::fs;
use regex::Regex;
//TODO: definir alias para regs

// vetor da memoria
#[derive(Debug)]
struct Instruction {
    opcode: Option<String>,
    funct3: Option<String>,
    funct7: Option<String>,
    imm: Option<String>,
    rd: Option<String>,
    rs1: Option<String>,
    rs2: Option<String>
}

#[derive(Debug)]
struct Definition {
    opcode: String,
    funct3: String,
    funct7: String,
    operation: String
}

fn parse_instructions(inst: &str) -> Instruction{
    let opcode = &inst[25..];
    // create instruction with opcode
    let mut instruction: Instruction = Instruction{
        opcode: Some(opcode.to_string()),
        funct3: None,
        funct7: None,
        imm: None,
        rd: None,
        rs1: None,
        rs2: None,
    };
    // estudar as_deref e unwrap/unwrap_or
    match opcode {
        // Tipo I
        "0000011" | "0010011" | "1100111" | "1110011" | "0000111" => {
            instruction.rd = Some(inst[20..24].to_string());
            instruction.funct3 = Some(inst[17..19].to_string());
            instruction.rs1 = Some(inst[12..16].to_string());
            instruction.imm = Some(inst[0..11].to_string());
        }
        
        // Tipo R
        "0110011" | "1010011" => {
            instruction.rd = Some(inst[20..24].to_string());
            instruction.funct3 = Some(inst[17..19].to_string());
            instruction.rs1 = Some(inst[12..16].to_string());
            instruction.rs2 = Some(inst[7..11].to_string());
            instruction.funct7 = Some(inst[0..6].to_string());
        }
        
        // Tipo S
        "0100011" | "0100111" => {
            instruction.funct3 = Some(inst[17..19].to_string());
            instruction.rs1 = Some(inst[12..16].to_string());
            instruction.rs2 = Some(inst[7..11].to_string());
            instruction.imm = Some(format!("{}{}",&inst[..6], &inst[20..24]).to_string());
        }

        // Tipo B
        "1100011" => {
            let imm_4_1 = &inst[20..23];
            let imm_11= inst.chars().nth(24).unwrap();
            let imm_10_5 = &inst[1..6];
            let imm_12= inst.chars().nth(0).unwrap();
            let imm = format!("{}{}{}{}",imm_12, imm_11, imm_10_5, imm_4_1);
            instruction.imm = Some(imm.to_string());
            instruction.funct3 = Some(inst[17..19].to_string());
            instruction.rs1 = Some(inst[12..16].to_string());
            instruction.rs2 = Some(inst[7..11].to_string());
        }

        // Tipo U
        "0010111" | "0110111" => {
            instruction.rd = Some(inst[20..24].to_string());
            instruction.imm = Some(inst[0..19].to_string());
        }
        
        // Tipo J
        "1101111" => {
            instruction.rd = Some(inst[20..24].to_string());
            let imm_20= inst.chars().nth(0).unwrap();
            let imm_10_1 = &inst[10..19];
            let imm_11= inst.chars().nth(9).unwrap();
            let imm_19_12 = &inst[1..8];
            let imm = format!("{}{}{}{}",imm_20, imm_10_1, imm_11, imm_19_12);
            instruction.imm = Some(imm.to_string());
        }
        &_ => todo!()
    }

    return instruction



}

fn parse_file(file_path: &str) -> Vec<Instruction> {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let vec = contents.split("\n");

    let mut parsed = Vec::new();

    for line in vec {
        let instruction: Instruction = parse_instructions(line);
        parsed.push(instruction);
    }

    return parsed
}
fn parse_def(file_path: &str) -> Vec<Definition>{
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let re = Regex::new(r"([0-9]{7})\s+([0-9]{3})\s+([0-9]{5}|\-{1})\s+([[:ascii:]].*)").unwrap();
    let mut defs = Vec::new();
    for cap in re.captures_iter(&contents){
        defs.push(
            Definition{
                opcode: cap[1].to_string(),
                funct3: cap[2].to_string(),
                funct7: cap[3].to_string(),
                operation: cap[4].to_string()
            }
        )
    }
    return defs
}
// faz parse das instrucoes em binario
fn process_inst(instructions: Vec<Instruction>) {
    let defs = parse_def("/home/caio/projetos/emulador-riscv/src/insts");
    println!("{:?}",defs);
    // for inst in instructions {
    // }
    //match para resolver a instrucao
}

fn main() {
    let parsed_file = parse_file("/home/caio/projetos/emulador-riscv/src/test.txt");
    // println!("{:?}",parsed_file);
    process_inst(parsed_file);
    // for line in 0..parsed_file.len() {
    //     println!("{}", parsed_file[line]);
    // }
}
