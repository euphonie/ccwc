use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
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

fn read_file<P: AsRef<Path>>(file_path: P) -> io::Result<(usize, usize)> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let (word_count, line_count) = reader.lines().fold(
        (0,0), |(word_acc, line_acc), line| {
            let line = line.unwrap();
            let words_in_line = line.split_whitespace().count();
            (word_acc + words_in_line, line_acc + 1)
        }
    );

    Ok((word_count, line_count))
}

fn print_all_info(file_path: &str){
    let Ok((word_count, line_count)) = read_file(file_path) else {panic!("Error reading metadata '{}'", file_path)};
    match fs::metadata(file_path) {
        Ok(metadata) => {
            println!("{} {} {} {}", line_count, word_count, metadata.len(), file_path);
        },
        Err(e) => {
            eprintln!("Error reading metadata for '{}': {}", file_path, e);
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
            eprintln!("Error reading metadata for '{}': {}", file_path, e);
            process::exit(1);
        }
    }
}