use chrono::Local;
use crate::command::handle_cmd;
use crate::storage::{load_history, save_line};

pub struct App
{
    pub input: String,
    pub history: Vec<(String, String)>,
    pub output: Vec<String>,
}

impl App
{
    pub fn new() -> Self
    {
        Self
        {
            input: String::new(),
            history: load_history(),
            output: Vec::new(),
        }
    }

    pub fn submit(&mut self) -> bool
    {
        let input = self.input.trim().to_string();
        self.input.clear();

        if input.is_empty()
        {
            return false;
        }

        if input == "quit" || input == "exit"
        {
            return true;
        }

        if input.starts_with('/')
        {
            handle_cmd(&input, &mut self.history, &mut self.output);
        }
        else
        {
            let timestamp = Local::now().format("%H:%M:%S").to_string();
            self.output.push(format!("[{}] {}", timestamp, input));
            self.history.push((timestamp.clone(), input.clone()));
            save_line(&timestamp, &input);
        }
        false
    }
}
