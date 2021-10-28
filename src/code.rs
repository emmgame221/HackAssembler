const DEST_OFS: u8 = 3;
const COMP_OFS: u8 = 6;

pub fn dest(mnem: &str) -> Result<u16, &'static str> {
    match mnem {
        "null" | "" => Ok(0b000 << DEST_OFS),
        "M" => Ok(0b001 << DEST_OFS),
        "D" => Ok(0b010 << DEST_OFS),
        "MD" => Ok(0b011 << DEST_OFS),
        "A" => Ok(0b100 << DEST_OFS),
        "AM" => Ok(0b101 << DEST_OFS),
        "AD" => Ok(0b110 << DEST_OFS),
        "AMD" => Ok(0b111 << DEST_OFS),
        _ => Err("Invalid dest mnemonic"),
    }
}

pub fn comp(mnem: &str) -> Result<u16, &'static str> {
    match mnem {
        "0" => Ok(0b0101010 << COMP_OFS),
        "1" => Ok(0b0111111 << COMP_OFS),
        "-1" => Ok(0b0111010 << COMP_OFS),
        "D" => Ok(0b0001100 << COMP_OFS),
        "A" => Ok(0b0110000 << COMP_OFS),
        "M" => Ok(0b1110000 << COMP_OFS),
        "!D" => Ok(0b0001101 << COMP_OFS),
        "!A" => Ok(0b0110001 << COMP_OFS),
        "!M" => Ok(0b1110001 << COMP_OFS),
        "-D" => Ok(0b0001111 << COMP_OFS),
        "-A" => Ok(0b0110011 << COMP_OFS),
        "-M" => Ok(0b1110011 << COMP_OFS),
        "D+1" => Ok(0b0011111 << COMP_OFS),
        "A+1" => Ok(0b0110111 << COMP_OFS),
        "M+1" => Ok(0b1110111 << COMP_OFS),
        "D-1" => Ok(0b0001110 << COMP_OFS),
        "A-1" => Ok(0b0110010 << COMP_OFS),
        "M-1" => Ok(0b1110010 << COMP_OFS),
        "D+A" => Ok(0b0000010 << COMP_OFS),
        "D+M" => Ok(0b1000010 << COMP_OFS),
        "D-A" => Ok(0b0010011 << COMP_OFS),
        "D-M" => Ok(0b1010011 << COMP_OFS),
        "A-D" => Ok(0b0000111 << COMP_OFS),
        "M-D" => Ok(0b1000111 << COMP_OFS),
        "D&A" => Ok(0b0000000 << COMP_OFS),
        "D&M" => Ok(0b1000000 << COMP_OFS),
        "D|A" => Ok(0b0010101 << COMP_OFS),
        "D|M" => Ok(0b1010101 << COMP_OFS),
        _ => Err("Invalid comp mnemonic"),
    }
}

pub fn jump(mnem: &str) -> Result<u16, &'static str> {
    match mnem {
        "null" | "" => Ok(0b000),
        "JGT" => Ok(0b001),
        "JEQ" => Ok(0b010),
        "JGE" => Ok(0b011),
        "JLT" => Ok(0b100),
        "JNE" => Ok(0b101),
        "JLE" => Ok(0b110),
        "JMP" => Ok(0b111),
        _ => Err("Invalid jump mnemonic"),
    }
}
