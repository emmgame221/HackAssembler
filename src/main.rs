use std::{env::args, fs::OpenOptions, io::Write};

mod parser;
use parser::*;
mod code;

const C_COM_PREF: u16 = 0b1110000000000000;

fn main() {
    let arg1 = match args().skip(1).next(){
        Some(s) => s,
        None => {
            println!("No argument provided.");
            return;
        }
    };
    if !arg1.ends_with(".asm") {
        println!("Please provide a .asm file for assembling.");
        return;
    }
    let mut infile = OpenOptions::new().read(true).open(&arg1).expect(&format!("{} is not a valid file.", arg1));
    let mut parser = Parser::new(&mut infile);
    let outfile_name = arg1.strip_suffix(".asm").unwrap().to_string() + ".hack";
    let mut outfile = OpenOptions::new().create(true).write(true).open(outfile_name).unwrap();
    while parser.has_more_commands() {
        parser.advance();
        match parser.command_type() {
            CommandType::ACommand => {
                if let Ok(num)  = parser.symbol().unwrap().parse::<u16>() {
                    writeln!(outfile, "{:016b}", num).unwrap();
                } else {
                    todo!()
                }
            }
            CommandType::CCommand => {
                let dest = match code::dest(&parser.dest().unwrap()) {
                    Ok(num) => num,
                    Err(e) => {
                        println!("{} at line {}", e, parser.line_num());
                        return;
                    }
                };
                let comp = match code::comp(&parser.comp().unwrap()) {
                    Ok(num) => num,
                    Err(e) => {
                        println!("{} at line {}", e, parser.line_num());
                        return;
                    }
                };
                let jump = match code::jump(&parser.jump().unwrap()) {
                    Ok(num) => num,
                    Err(e) => {
                        println!("{} at line {}", e, parser.line_num());
                        return;
                    }
                };
                let to_write = C_COM_PREF | dest | comp | jump;
                writeln!(outfile, "{:016b}", to_write).unwrap();
            }
            CommandType::LCommand => todo!(),
            CommandType::None => todo!(),
        }
    }
}