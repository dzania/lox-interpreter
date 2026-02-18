use std::{env, fs, io, io::BufRead, io::Write, path::Path};
pub mod scanner;
pub mod token;

fn run_file(path: &str) -> i32 {
    let file_content = fs::read_to_string(Path::new(path)).expect("Error reading file");
    run(file_content);
    std::process::exit(0)
}

fn run_prompt() {
    let stdin = io::stdin();
    loop {
        print!(">> ");
        io::stdout().flush().expect("Failed to flush output");
        let mut line = String::new();
        stdin
            .lock()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line.is_empty() {
            break;
        }
    }
}

fn run(source: String) {
    let scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();

    tokens.iter().for_each(|token| println!("{token:#?}"));
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, place: &str, message: &str) {
    eprintln!("[line {line} ] Error {place}: {message}");
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    match args.len() {
        1 => run_prompt(),
        2 => {
            run_file(&args[1]);
        }
        _ => std::process::exit(64),
    };
}
