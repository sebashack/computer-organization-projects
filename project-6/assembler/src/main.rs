use assembler::*;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error: Hack assembly file is needed.");
        println!("Usage: cargo run <filename>.asm");
        std::process::exit(0);
    }

    let input_file_name = args.get(1).unwrap();
    let mut symbols = SymbolTable::new();
    let mut parser = Parser::new(PathBuf::from(input_file_name));
    let mut counter = 0;

    while parser.has_more_commands() {
        parser.advance();

        if parser.command_type() == 'L' {
            symbols.add_entry(parser.symbol().as_str(), counter);
        } else {
            counter += 1;
        }
    }

    parser.restart();

    let output_file_name = input_file_name.replace(".asm", ".hack");
    let mut file = File::create(&output_file_name).unwrap();
    let coder = Code::new();

    let mut variable = 16;
    while parser.has_more_commands() {
        parser.advance();

        match parser.command_type() {
            'A' => {
                let mut num = 0;
                let mut symbol = parser.symbol();

                if is_int(&symbol) {
                    num = symbol.parse::<u32>().unwrap();
                } else if symbols.contains(symbol.as_str()) {
                    num = symbols.get_address(symbol.as_str());
                } else {
                    num = variable;
                    symbols.add_entry(symbol.as_str(), num);
                    variable += 1;
                }
                file.write_all(format!("{:016b}", num).as_bytes());
                file.write_all("\n".as_bytes());
            }
            _ => {
                let cmp = coder.comp(parser.comp().as_str());
                let dest = coder.dest(parser.dest().as_str());
                let jump = coder.jump(parser.jump().as_str());
                file.write_all(format!("111{}{}{}", cmp, dest, jump).as_bytes());
                file.write_all("\n".as_bytes());
            }
        }
    }
}

fn is_int(s: &String) -> bool {
    let test = s.parse::<u32>();
    match test {
        Ok(ok) => true,
        Err(e) => false,
    }
}
