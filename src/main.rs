use parser::*;
use std::{env::args, fs::OpenOptions, io::Write};

mod code;
mod parser;
mod symbol_table;

const C_COM_PREF: u16 = 0b1110000000000000;

fn main() {
    let arg1 = match args().skip(1).next() {
        Some(s) => s,
        None => {
            println!("No argument provided.");
            return;
        }
    };
    let mut debug = false;
    if let Some(s) = args().next() {
        if s.contains('-') {
            debug = true;
        }
    }
    if !arg1.ends_with(".asm") {
        println!("Please provide a .asm file for assembling.");
        return;
    }
    let mut infile = OpenOptions::new()
        .read(true)
        .open(&arg1)
        .expect(&format!("{} is not a valid file.", arg1));
    let mut parser = Parser::new(&mut infile, debug);
    let outfile_name = arg1.strip_suffix(".asm").unwrap().to_string() + ".hack";
    let mut outfile = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(outfile_name)
        .unwrap();
    let mut sym_table = symbol_table::SymbolTable::new();
    let mut next_address: u16 = 0;
    // First pass, just resolve all labels
    while parser.has_more_commands() {
        parser.advance();
        match parser.command_type() {
            CommandType::ACommand | CommandType::CCommand => next_address += 1,
            CommandType::LCommand => {
                let symbol = parser.symbol().unwrap();
                if sym_table.contains(&symbol) {
                    println!("Duplicate label at line {}", parser.line_num());
                    return;
                } else {
                    sym_table.add_entry(symbol, next_address);
                }
            }
            _ => {}
        }
    }
    // Prepare for the next pass
    parser.reset();
    let mut next_var: u16 = 16;
    while parser.has_more_commands() {
        parser.advance();
        match parser.command_type() {
            CommandType::ACommand => {
                let symbol = parser.symbol().unwrap();
                if let Ok(num) = symbol.parse::<u16>() {
                    writeln!(outfile, "{:016b}", num).unwrap();
                } else {
                    if sym_table.contains(&symbol) {
                        writeln!(outfile, "{:016b}", sym_table.get_address(&symbol)).unwrap();
                    } else {
                        sym_table.add_entry(symbol, next_var);
                        writeln!(outfile, "{:016b}", next_var).unwrap();
                        next_var += 1;
                    }
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
            _ => {}
        }
    }
}
