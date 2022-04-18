use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;

pub struct Parser {
    cmd: String,
    current: i32,
    commands: Vec<String>,
}

impl Parser {
    pub fn new(file_path: PathBuf) -> Self {
        let lines = read_lines(file_path.as_path());
        let mut commands = Vec::new();
        for line in lines {
            let line = line.unwrap().trim().replace(" ", "");
            commands.push(line);
        }

        Parser {
            cmd: String::new(),
            current: -1,
            commands,
        }
    }

    pub fn has_more_commands(&self) -> bool {
        self.current + 1 < self.commands.len().try_into().unwrap()
    }

    pub fn advance(&mut self) {
        self.current += 1;
        self.cmd = self
            .commands
            .get(self.current as usize)
            .unwrap()
            .to_string();
    }

    pub fn restart(&mut self) {
        self.cmd = String::new();
        self.current = -1;
    }

    pub fn command_type(&self) -> char {
        match self.cmd.as_str() {
            "@" => 'A',
            "(" => 'L',
            _ => 'C',
        }
    }

    pub fn symbol(&self) -> String {
        match self.command_type() {
            'A' => self.cmd.split_at(1).0.to_string(),
            'L' => self.cmd[1..self.cmd.len() - 1].to_string(),
            _ => String::new(),
        }
    }

    pub fn dest(&self) -> String {
        if self.command_type() == 'C' {
            let mut tmp = self.cmd.clone();
            if tmp.contains("=") {
                tmp = tmp.split_once("=").unwrap().0.to_string();
            }

            return tmp;
        } else {
            return String::new();
        }
    }

    pub fn comp(&self) -> String {
        if self.command_type() == 'C' {
            let mut tmp = self.cmd.clone();
            if tmp.contains("=") {
                let l = tmp.split_once("=").unwrap().1.to_string();
                tmp = l.split_once(";").unwrap().0.to_string();
            }

            return tmp;
        } else {
            return String::new();
        }
    }

    pub fn jump(&self) -> String {
        if self.command_type() == 'C' {
            let mut tmp = self.cmd.clone();
            if tmp.contains("=") {
                let l = tmp.split_once("=").unwrap().1.to_string();
                tmp = l.split_once(";").unwrap().1.to_string();
            }

            return tmp;
        } else {
            return String::new();
        }
    }
}

pub struct Code {
    d_table: HashMap<String, String>,
    c_table: HashMap<String, String>,
    j_table: HashMap<String, String>,
}

impl Code {
    pub fn new() -> Self {
        let mut d_table = HashMap::new();

        d_table.insert("".to_string(), "000".to_string());
        d_table.insert("M".to_string(), "001".to_string());
        d_table.insert("D".to_string(), "010".to_string());
        d_table.insert("MD".to_string(), "011".to_string());
        d_table.insert("A".to_string(), "100".to_string());
        d_table.insert("AM".to_string(), "101".to_string());
        d_table.insert("AD".to_string(), "110".to_string());
        d_table.insert("AMD".to_string(), "111".to_string());

        let mut c_table = HashMap::new();

        c_table.insert("0".to_string(), "0101010".to_string());
        c_table.insert("1".to_string(), "0111111".to_string());
        c_table.insert("-1".to_string(), "0111010".to_string());
        c_table.insert("D".to_string(), "0001100".to_string());
        c_table.insert("A".to_string(), "0110000".to_string());
        c_table.insert("M".to_string(), "1110000".to_string());
        c_table.insert("!D".to_string(), "0001101".to_string());
        c_table.insert("!A".to_string(), "0110001".to_string());
        c_table.insert("!M".to_string(), "1110001".to_string());
        c_table.insert("-D".to_string(), "0001111".to_string());
        c_table.insert("-A".to_string(), "0110011".to_string());
        c_table.insert("-M".to_string(), "1110011".to_string());
        c_table.insert("D+1".to_string(), "0011111".to_string());
        c_table.insert("A+1".to_string(), "0110111".to_string());
        c_table.insert("M+1".to_string(), "1110111".to_string());
        c_table.insert("D-1".to_string(), "0001110".to_string());
        c_table.insert("A-1".to_string(), "0110010".to_string());
        c_table.insert("M-1".to_string(), "1110010".to_string());
        c_table.insert("D+A".to_string(), "0000010".to_string());
        c_table.insert("D+M".to_string(), "1000010".to_string());
        c_table.insert("D-A".to_string(), "0010011".to_string());
        c_table.insert("D-M".to_string(), "1010011".to_string());
        c_table.insert("A-D".to_string(), "0000111".to_string());
        c_table.insert("M-D".to_string(), "1000111".to_string());
        c_table.insert("D&A".to_string(), "0000000".to_string());
        c_table.insert("D&M".to_string(), "1000000".to_string());
        c_table.insert("D|A".to_string(), "0010101".to_string());
        c_table.insert("D|M".to_string(), "1010101".to_string());

        let mut j_table = HashMap::new();

        j_table.insert("".to_string(), "000".to_string());
        j_table.insert("JGT".to_string(), "001".to_string());
        j_table.insert("JEQ".to_string(), "010".to_string());
        j_table.insert("JGE".to_string(), "011".to_string());
        j_table.insert("JLT".to_string(), "100".to_string());
        j_table.insert("JNE".to_string(), "101".to_string());
        j_table.insert("JLE".to_string(), "110".to_string());
        j_table.insert("JMP".to_string(), "111".to_string());

        Code {
            d_table,
            c_table,
            j_table,
        }
    }

    pub fn dest(&self, token: &str) -> String {
        self.d_table[&token.to_string()].clone()
    }

    pub fn comp(&self, token: &str) -> String {
        self.c_table[&token.to_string()].clone()
    }

    pub fn jump(&self, token: &str) -> String {
        self.j_table[&token.to_string()].clone()
    }
}

pub struct SymbolTable {
    table: HashMap<String, u32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        table.insert("R0".to_string(), 0);
        table.insert("R1".to_string(), 1);
        table.insert("R2".to_string(), 2);
        table.insert("R3".to_string(), 3);
        table.insert("R4".to_string(), 4);
        table.insert("R5".to_string(), 5);
        table.insert("R6".to_string(), 6);
        table.insert("R7".to_string(), 7);
        table.insert("R8".to_string(), 8);
        table.insert("R9".to_string(), 9);
        table.insert("R10".to_string(), 10);
        table.insert("R11".to_string(), 11);
        table.insert("R12".to_string(), 12);
        table.insert("R13".to_string(), 13);
        table.insert("R14".to_string(), 14);
        table.insert("R15".to_string(), 15);
        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24576);
        table.insert("SP".to_string(), 0);
        table.insert("LCL".to_string(), 1);
        table.insert("ARG".to_string(), 2);
        table.insert("THIS".to_string(), 3);
        table.insert("THAT".to_string(), 4);

        SymbolTable { table }
    }

    pub fn add_entry(&mut self, token: &str, value: u32) {
        self.table.insert(token.to_string(), value);
    }

    pub fn contains(&self, token: &str) -> bool {
        self.table.contains_key(&token.to_string())
    }

    pub fn get_address(&self, token: &str) -> u32 {
        *self.table.get(&token.to_string()).unwrap()
    }
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
