use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Parser {
    lines: Vec<String>,
    cur_line: usize,
    cur_com_type: CommandType,
    cur_symbol: Option<String>,
    cur_dest: Option<String>,
    cur_comp: Option<String>,
    cur_jump: Option<String>,
}

impl Parser {
    pub fn new(file: &mut File) -> Self {
        let reader = BufReader::new(file);
        let lines = reader
            .lines()
            .map(|x| x.expect("Error while reading file").trim().to_string())
            .filter(|x| !x.starts_with("//") && !(x == "") && !(x == "\n"))
            .collect::<Vec<String>>();
        Parser {
            lines,
            cur_line: 0,
            cur_com_type: CommandType::None,
            cur_symbol: None,
            cur_dest: None,
            cur_comp: None,
            cur_jump: None,
        }
    }

    pub fn has_more_commands(&self) -> bool {
        self.cur_line < self.lines.len()
    }

    pub fn advance(&mut self) {
        let line = &self.lines[self.cur_line];
        if line.starts_with("@") {
            self.cur_com_type = CommandType::ACommand;
            self.cur_symbol = Some(line.strip_prefix("@").unwrap().to_string());
            self.cur_dest = None;
            self.cur_comp = None;
            self.cur_jump = None;
        } else if line.starts_with("(") && line.ends_with(")") {
            self.cur_com_type = CommandType::LCommand;
            self.cur_symbol = Some(line.trim_matches(|x| x == '(' || x == ')').to_string());
            self.cur_dest = None;
            self.cur_comp = None;
            self.cur_jump = None;
        } else {
            self.cur_com_type = CommandType::CCommand;
            self.cur_symbol = None;
            let eq_idx = line.find('=');
            match eq_idx {
                Some(i) => {
                    self.cur_dest = Some(line[0..i].to_string());
                }
                None => {
                    self.cur_dest = Some("null".to_string());
                }
            }
            let semi_idx = line.find(';');
            match semi_idx {
                Some(i) => {
                    self.cur_jump = Some(line[i + 1..].to_string());
                }
                None => {
                    self.cur_jump = Some("null".to_string());
                }
            }
            let comp_start = match eq_idx {
                Some(i) => i + 1,
                None => 0,
            };
            let comp_end = match semi_idx {
                Some(i) => i,
                None => line.len(),
            };
            self.cur_comp = Some(line[comp_start..comp_end].to_string());
        }
        self.cur_line += 1;
    }

    pub fn command_type(&self) -> CommandType {
        self.cur_com_type
    }

    pub fn symbol(&self) -> Option<String> {
        self.cur_symbol.clone()
    }

    pub fn dest(&self) -> Option<String> {
        self.cur_dest.clone()
    }

    pub fn comp(&self) -> Option<String> {
        self.cur_comp.clone()
    }

    pub fn jump(&self) -> Option<String> {
        self.cur_jump.clone()
    }

    pub fn line_num(&self) -> usize {
        self.cur_line
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CommandType {
    ACommand,
    CCommand,
    LCommand,
    None,
}
