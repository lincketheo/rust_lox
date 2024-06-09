use std::env;
use std::fs;
use std::io;
use std::process::ExitCode;

mod models;
mod lexer;

use models::CompilationFailure;

/// Usage:
/// $ ./lox <filename> # compiles <filename>
/// $ ./lox # Enters into a REPL for lox
fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        ExitCode::FAILURE
    } else if args.len() == 1 {
        run_prompt() // REPL
    } else {
        run_file(&args[1]) // Compile file
    }
}

/// Compiles a file. Returns success or failure
///
/// * `file_name`: The file to compile
fn run_file(file_name: &String) -> ExitCode {
    let data = match fs::read_to_string(file_name) {
        Ok(data) => data,
        Err(msg) => {
            eprintln!("Failed to read file: {}. Error: {:?}\n", file_name, msg);
            return ExitCode::FAILURE;
        }
    };

    match run_string(&data) {
        Ok(_) => ExitCode::SUCCESS,
        Err(errs) => {
            errs.iter().for_each(|x| x.print_error());
            ExitCode::FAILURE
        }
    }
}

fn run_prompt() -> ExitCode {
    loop {
        print!(">> ");
        let mut input = String::new();

        // Flush the input
        if let Err(e) = io::Write::flush(&mut io::stdout()) {
            eprintln!("Failed to flush. Error: {:?}\n", e);
            return ExitCode::FAILURE;
        }

        // Read input line
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Failed to flush. Error: {:?}\n", e);
            return ExitCode::FAILURE;
        }

        // Exit REPL
        if input.trim() == "exit" {
            println!("Goodbye!");
            return ExitCode::SUCCESS;
        }

        // Execute command
        match run_string(&input) {
            Ok(_) => {}
            Err(errs) => {
                for err in errs.into_iter() {
                    err.print_error();
                }
            }
        };
    }
}

fn run_string(data: &String) -> Result<(), Vec<CompilationFailure>> {
    dbg!(data);
    Ok(())
}
