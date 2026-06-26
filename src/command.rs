pub fn handle_cmd(cmd: &str, history: &mut Vec<(String, String)>, output: &mut Vec<String>)
{
    match cmd
    {
        "/clear" => history.clear(),
        "/history" =>
        {
            for (i, (timestamp, line)) in history.iter().enumerate()
            {
                output.push(format!("{}: [{}] {}", i+1, timestamp, line));
            }
        }
        "/help" =>
        {
            output.push("commands: /clear, /history, /help, /quit".to_string());
        }
        "/quit" =>
        {
            std::process::exit(0);
        }
        _ => output.push(format!("unknown command: {}", cmd)),
    }
}
