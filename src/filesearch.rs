use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
  Author Gaurav Sablok
  SLB Potsdam
  Date : 2025-2-5

 a general purpose function for reading and writing fasta vectors.

*/

pub fn read(path: &str) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut header: Vec<String> = Vec::new();
    let mut sequence: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not present");
        if line.starts_with(">") {
            header.push(line.replace(">", ""));
        } else if !line.starts_with(">") {
            sequence.push(line);
        }
    }
    Ok((header, sequence))
}
