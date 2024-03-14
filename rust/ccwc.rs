use std::env;
use std::process;

mod file_reader;
use file_reader::FileReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!(
            "Usage: {} [OPTION] <file_path> \n
            \n
            OPTIONS:\n
                -c, --bytes\n
                    print the byte counts\n
                -l, --lines\n
                    print the newline counts\n
            ",
            args[0]
        );
        process::exit(1);
    }

    let mode = get_mode(&args);
    let file_path = &args[args.len() -1];

    let file_reader = FileReader::new(String::from(file_path));
    process_file(mode, &file_reader);    
}

fn process_file(mode: &str, file_reader: &FileReader) {
    match mode {
        "all" => print_all_info(file_reader),
        "bytes" => print_bytes(file_reader),
        "lines" => print_lines(file_reader),
        "" => print_all_info(file_reader),
        _ => unreachable!(),
    }
}

fn get_mode(args: &[String]) -> &str {
    if args.len() == 3 {
        match args[1].as_str() {
            "-c" | "--bytes" => "bytes",
            "-l" | "--lines" => "lines",
            _ => "unknown",
        }
    } else {
        "all"
    }
}

fn print_all_info(file_reader: &FileReader) {
    let Ok(byte_count) = file_reader.count_bytes() else { 
        panic!("Error reading size of file for '{}'", file_reader.file_path)
    };
    match file_reader.count_lines_and_words() {
        Ok((line_count, word_count)) => {
              println!(
                "{} {} {} {}",
                line_count,
                word_count,
                byte_count,
                file_reader.file_path
            );
        }
        Err(e) => {
            eprintln!("Error reading metadata for '{}': {}", file_reader.file_path, e);
            process::exit(1);
        }
    }
}

fn print_bytes(file_reader: &FileReader) {
    match file_reader.count_bytes() {
        Ok(byte_count) => {
            println!("{} {}", byte_count, file_reader.file_path);
        }
        Err(e) => {
            eprintln!("Error reading metadata for '{}': {}", file_reader.file_path, e);
            process::exit(1);
        }
    }
}

fn print_lines(file_reader: &FileReader) {
    match file_reader.count_lines() {
        Ok(line_count) => {
            println!("{} {}", line_count, file_reader.file_path);
        }
        Err(e) => {
            eprintln!("Error reading metadata for '{}': {}", file_reader.file_path, e);
            process::exit(1);
        }
    }
}
