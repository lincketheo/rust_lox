
use std::env;
use std::process::ExitCode;
use std::fs;
use std::io;

mod models;
mod lexer;

use models::CompilationFailure;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        ExitCode::FAILURE
    } else if args.len() == 1 {
        run_prompt()
    } else {
        run_file(&args[1])
    }
}

fn run_file(file_name : &String) -> ExitCode {
    match fs::read_to_string(file_name) {
        Result::Ok(data) => { 
            match run_string(&data) {
                Ok(_) => {
                    ExitCode::SUCCESS
                },
                Err(errs) => {
                    errs.iter().for_each(|x| x.print_error());
                    ExitCode::FAILURE
                }
            }
        },
        Result::Err(error) => {
            eprintln!("Failed to open file {}. Reason: {error}", file_name);
            ExitCode::FAILURE 
        }
    }
}

fn run_prompt() -> ExitCode {
    loop {
        print!(">> ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");

        let mut input = String::new();

        let _ = match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "exit" {
                    return ExitCode::SUCCESS;
                } else {
                    run_string(&input) 
                }
            },
            Err(error) => {
                eprintln!("Error reading line: {error}");
                return ExitCode::FAILURE;
            }
        };
    };
}

fn run_string(data: &String) -> Result<(), Vec<CompilationFailure>> {
    dbg!(data);
    Ok(())
}



