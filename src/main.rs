use std::{env, fs, io, io::BufRead, io::Write, path::Path};

pub mod expr;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod token;

use interpreter::{Interpreter, RuntimeError};
use scanner::ScanError;

// ── Error type ───────────────────────────────────────────────────────────────

/// Distinguishes the two failure modes so run_file can exit with the right code.
enum RunError {
    Compile, // exit 65
    Runtime, // exit 70
}

// ── Lox application struct ───────────────────────────────────────────────────

struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    fn new() -> Self {
        Self {
            interpreter: Interpreter,
        }
    }

    fn run_file(&mut self, path: &str) {
        let source = fs::read_to_string(Path::new(path)).expect("Error reading file");
        match self.run(source) {
            Ok(()) => {}
            Err(RunError::Compile) => std::process::exit(65),
            Err(RunError::Runtime) => std::process::exit(70),
        }
    }

    fn run_prompt(&mut self) {
        let stdin = io::stdin();
        loop {
            print!(">> ");
            io::stdout().flush().expect("Failed to flush output");
            let mut line = String::new();
            if stdin
                .lock()
                .read_line(&mut line)
                .expect("Failed to read line")
                == 0
            {
                break; // EOF
            }
            // Errors are already printed; we just keep going.
            let _ = self.run(line);
        }
    }

    fn run(&mut self, source: String) -> Result<(), RunError> {
        let (tokens, scan_errors) = scanner::Scanner::new(source).scan_tokens();

        if !scan_errors.is_empty() {
            for ScanError { line, message } in &scan_errors {
                eprintln!("[line {line}] Error: {message}");
            }
            return Err(RunError::Compile);
        }

        let mut parser = parser::Parser::new(tokens);
        let statements = parser.parse();
        if parser.had_error() {
            return Err(RunError::Compile);
        }

        self.interpreter
            .interpret(statements)
            .map_err(|RuntimeError { token, message }| {
                eprintln!("{message}\n[line {}]", token.line);
                RunError::Runtime
            })?;

        Ok(())
    }
}

// ── Entry point ──────────────────────────────────────────────────────────────

fn main() {
    let mut lox = Lox::new();
    let args = env::args().collect::<Vec<_>>();
    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]),
        _ => std::process::exit(64),
    }
}
