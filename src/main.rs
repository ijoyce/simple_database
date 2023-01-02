use std::io;
use std::process::exit;

enum MetaCommandResult {
    Success,
    Unrecognized,
}

#[derive(Debug)]
enum StatementType {
    Insert,
    Select,
}

fn main() -> io::Result<()> {
    loop {
        print_prompt();
        let buffer = read_input()?;

        if buffer.chars().next().unwrap() == '.' {
            match do_meta_command(&buffer) {
                MetaCommandResult::Success => {}
                MetaCommandResult::Unrecognized => {
                    println!("Unrecognized command {}", buffer);
                    continue;
                }
            }
        }

        let statement = prepare_statement(&buffer);

        match statement {
            Some(StatementType::Insert) => {},
            Some(StatementType::Select) => {},
            None => {
                println!("Unrecognized keyword at the start of {}", buffer);
                continue;
            }
        }

        execute_statement(statement.unwrap());
    }
}

fn execute_statement(statement: StatementType) {
    match statement {
        StatementType::Insert => { println!("This is where we'd do an insert.")},
        StatementType::Select => { println!("This is where we'd do a select.")},
    }
}

fn prepare_statement(buffer: &str) -> Option<StatementType> {
    if buffer.starts_with("insert") {
        return Some(StatementType::Insert);
    }

    if buffer.starts_with("select") {
        return Some(StatementType::Select);
    }

    None
}

fn do_meta_command(buffer: &str) -> MetaCommandResult {
    match buffer {
        ".exit" => exit(0),
        _ => MetaCommandResult::Unrecognized,
    }
}

fn read_input() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    buffer.pop(); // remove \n
    Ok(buffer)
}

fn print_prompt() {
    print!("db > ");
}
