use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn handle_cmd(cmd: &str, history: &mut Vec<(String, String)>)
{
    match cmd
    {
        "/clear" => history.clear(),
        "/history" =>
        {
            for (i, (timestamp, line)) in history.iter().enumerate()
            {
                println!("{}: [{}] {}", i+1, timestamp, line);
            }
        }
        "/help" =>
        {
            println!("commands: /clear, /history, /help, /quit");
        }
        "/quit" =>
        {
            println!("goodbye");
            std::process::exit(0);
        }
        _ => println!("unknown command: {}", cmd),
    }
}

fn load_history() -> Vec<(String, String)>
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

fn save_line(timestamp: &str, line: &str)
{
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("history.txt")
        .unwrap();
    writeln!(file, "{}|{}", timestamp, line).unwrap();
}

fn main()
{
    let mut history = load_history();

    loop
    {
        print!("~|> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.starts_with('/')
        {
            handle_cmd(input, &mut history);
            continue;
        }
        else
        {
            let timestamp = Local::now().format("%H:%M:%S").to_string();
            history.push((timestamp.clone(), input.to_string()));
            save_line(&timestamp, input);
        }
    }
}