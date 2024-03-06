use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} [-c|--bytes] <file_path>", args[0]);
        process::exit(1);
    }

    let (mode, file_path_index) = get_mode(&args);
    let file_path = &args[file_path_index];

    match mode {
        Some("all") => print_all_info(file_path),
        Some("bytes") => print_bytes(file_path),
        None => print_all_info(file_path),
        _ => unreachable!(),
    }
}

fn get_mode(args: &[String]) -> (Option<&str>, usize) {
    if args.len() == 3 {
        match args[1].as_str() {
            "-c" | "--bytes" => (Some("bytes"), 2),
            _ => (Some("all"), 1),
        }
    } else {
        (None, 1)
    }
}

fn get_file_content(file_path: &str) -> (usize, usize) {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let word_count = content.split_whitespace().count();
            let line_count = content.lines().count() - 1;
            (word_count, line_count)
        },
        Err(e) => {
            eprintln!("Error reading metadata for '{}': {}", file_path, e);
            process::exit(1);
        }
    }
}

fn print_all_info(file_path: &str){
    let (word_count, line_count) = get_file_content(file_path);
    match fs::metadata(file_path) {
        Ok(metadata) => {
            println!("{} {} {} {}", line_count, word_count, metadata.len(), file_path);
        },
        Err(e) => {
            eprintln!("Error reading metadat for '{}': {}", file_path, e);
            process::exit(1);
        }
    }
}

fn print_bytes(file_path: &str) {
    match fs::metadata(file_path) {
        Ok(metadata) => {
            println!("{} {}", metadata.len(), file_path);
        },
        Err(e) => {
            eprintln!("Error reading metadat for '{}': {}", file_path, e);
            process::exit(1);
        }
    }
}