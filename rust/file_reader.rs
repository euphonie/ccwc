use std::fs;
use std::fs::File;
use std::io::{self, BufRead};

pub struct FileReader {
    pub file_path: String,
}

impl FileReader {
    pub fn new(file_path: String) -> Self {
        FileReader { file_path }
    }

    #[allow(dead_code)]
    fn read_file(&self) -> Result<Vec<String>, io::Error> {
        let file = File::open(&self.file_path)?;
        let reader = io::BufReader::new(file);
        let lines = reader.lines().collect::<Result<Vec<_>, io::Error>>()?;
        Ok(lines)
    }

    pub fn count_lines(&self) -> Result<usize, io::Error> {
        let file = File::open(&self.file_path)?;
        let mut reader = io::BufReader::new(file);
        let mut lines = 0;
        let mut buffer = String::new();
        while reader.read_line(&mut buffer)? > 0 {
            lines += 1;
        }
        Ok(lines)
    }

    pub fn count_lines_and_words(&self) -> Result<(usize, usize), io::Error> {
        let file = File::open(&self.file_path)?;
        let reader = io::BufReader::new(file);
        let (lines, words) = reader.lines().fold((0, 0), |(line_acc, word_acc), line| {
            let line = line.unwrap();
            let words_in_line = line.split_whitespace().count();
            (line_acc + 1, word_acc + words_in_line)
        });
        Ok((lines, words))
    }

    pub fn count_bytes(&self) -> Result<u64, io::Error> {
        let metadata = fs::metadata(&self.file_path).unwrap();
        Ok(metadata.len())
    }
}
