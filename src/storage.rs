use std::fs::{self, OpenOptions};
use std::io::{self, Write};

pub fn load_history() -> Vec<(String, String)>
{
    fs::read_to_string("history.txt")
        .unwrap_or_default()
        .lines()
        .filter_map(|l| 
            {
                let mut parts = l.splitn(2, '|');
                Some((parts.next()?.to_string(), parts.next()?.to_string()))
            })
        .collect()
}

pub fn save_line(timestamp: &str, line: &str)
{
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("history.txt")
        .unwrap();
    writeln!(file, "{}|{}", timestamp, line).unwrap();
}