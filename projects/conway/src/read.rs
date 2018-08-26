use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn read_from_file(path: String) -> Vec<Vec<bool>> {
    let file = File::open(path).expect("file not found");
    let file = BufReader::new(&file);
    let mut lines = Vec::new();
    for line in file.lines() {
        let line = line
            .expect("Error")
            .as_str()
            .chars()
            .map(|x| if x == '1' {
            return true;
        } else {
            return false;
            }).collect();
        lines.push(line);
    }
    lines
}
