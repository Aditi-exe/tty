pub fn handle_cmd(cmd: &str, history: &mut Vec<(String, String)>, utility: &mut Vec<String>)
{
    match cmd
    {
        "/history" =>
        {
            if history.is_empty() {
                utility.push("no history yet".to_string());
            } else {
                for (i, (timestamp, line)) in history.iter().enumerate()
                {
                    utility.push(format!("{}: [{}] {}", i + 1, timestamp, line));
                }
            }
        }
        "/sessions" =>
        {
            let sessions = crate::storage::list_sessions();
            if sessions.is_empty() {
                utility.push("no sessions found".to_string());
            } else {
                utility.push("sessions:".to_string());
                for s in sessions {
                    utility.push(format!("  {}", s));
                }
            }
        }
        "/help" =>
        {
            utility.push("commands: /clear  /history  /sessions  /session <name>  /color <name>  /help  quit".to_string());
            utility.push("colors:   red  green  yellow  blue  magenta  cyan  white  gray".to_string());
        }
        "/quit" =>
        {
            std::process::exit(0);
        }
        _ => utility.push(format!("unknown command: {}", cmd)),
    }
}
