use chrono::Local;
use crate::command::handle_cmd;
use crate::storage::{load_history, save_line};

pub struct App
{
    pub input: String,
    pub history: Vec<(String, String)>,
    pub output: Vec<String>,
    // TODO: add scroll: u16 and visible_height: usize for mouse scroll on output
    pub history_index: Option<usize>,
    pub saved_input: String,
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
            history_index: None,
            saved_input: String::new(),
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

    pub fn history_up(&mut self) {
        if self.history.is_empty()
        {
            return;
        }
        let new_index = match self.history_index
        {
            None => 
            {
                self.saved_input = self.input.clone();
                self.history.len() - 1
            }
            Some(i) => i.saturating_sub(1),
        };
        self.history_index = Some(new_index);
        self.input = self.history[new_index].1.clone();
    }

    pub fn history_down(&mut self) {
        match self.history_index {
            None => {}
            Some(i) => {
                if i + 1 >= self.history.len() {
                    self.history_index = None;
                    self.input = self.saved_input.clone();
                } else {
                    self.history_index = Some(i + 1);
                    self.input = self.history[i + 1].1.clone();
                }
            }
        }
    }
}
