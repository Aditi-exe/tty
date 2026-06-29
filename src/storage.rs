use std::fs::{self, OpenOptions};
use std::io::Write;

pub fn ensure_session_file(session: &str)
{
    let path = format!("history_{}.txt", session);
    if !std::path::Path::new(&path).exists() {
        let _ = std::fs::File::create(&path);
    }
}

pub fn list_sessions() -> Vec<String>
{
    let Ok(entries) = fs::read_dir(".") else { return vec![]; };
    entries
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().into_string().ok()?;
            let session = name.strip_prefix("history_")?.strip_suffix(".txt")?;
            Some(session.to_string())
        })
        .collect()
}

pub fn load_history(session: &str) -> Vec<(String, String)>
{
    fs::read_to_string(format!("history_{}.txt", session))
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let mut parts = l.splitn(2, '|');
            Some((parts.next()?.to_string(), parts.next()?.to_string()))
        })
        .collect()
}

pub fn save_line(session: &str, timestamp: &str, line: &str)
{
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("history_{}.txt", session))
        .unwrap();
    writeln!(file, "{}|{}", timestamp, line).unwrap();
}
