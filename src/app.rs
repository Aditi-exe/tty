use chrono::Local;
use crate::command::handle_cmd;
use crate::constants::{COLORS, COMMANDS};
use crate::storage::{ensure_session_file, load_history, list_sessions, save_line};
use crate::utils::char_to_byte;

pub struct App
{
    pub input: String,
    pub cursor_pos: usize,
    pub history: Vec<(String, String)>,
    pub nav_history: Vec<String>,
    pub output: Vec<String>,
    pub utility: Vec<String>,
    pub scroll: u16,
    pub visible_height: usize,
    pub history_index: Option<usize>,
    pub saved_input: String,
    pub session: String,
    pub session_color: String,
    pub suggestion: Option<String>,
    pub message: Option<String>,
    pub show_art: bool,
}

impl App
{
    pub fn new() -> Self
    {
        let session = "default".to_string();
        ensure_session_file(&session);
        let history = load_history(&session);
        let nav_history: Vec<String> = history.iter().map(|(_, t)| t.clone()).collect();

        Self
        {
            input: String::new(),
            cursor_pos: 0,
            history,
            nav_history,
            output: Vec::new(),
            utility: Vec::new(),
            scroll: 0,
            visible_height: 0,
            history_index: None,
            saved_input: String::new(),
            session,
            session_color: "cyan".to_string(),
            suggestion: None,
            message: None,
            show_art: true,
        }
    }

    pub fn submit(&mut self) -> bool
    {
        let input = self.input.trim().to_string();
        self.input.clear();
        self.cursor_pos = 0;
        self.history_index = None;
        self.suggestion = None;
        self.message = None;
        self.utility.clear();
        self.scroll = 0;

        if input.is_empty() { return false; }
        if input == "quit" || input == "exit" { return true; }

        self.nav_history.push(input.clone());

        if input == "/clear" {
            self.history.clear();
            self.output.clear();
            return false;
        }

        if input.starts_with("/session ") {
            let name = input.trim_start_matches("/session ").trim().to_string();
            if !name.is_empty() {
                self.session = name.clone();
                ensure_session_file(&self.session);
                self.history = load_history(&self.session);
                self.nav_history = self.history.iter().map(|(_, t)| t.clone()).collect();
                self.message = Some(format!("switched to session '{}'", name));
            }
            return false;
        }

        if input.starts_with("/color ") {
            let color = input.trim_start_matches("/color ").trim().to_string();
            if COLORS.contains(&color.as_str()) {
                self.session_color = color.clone();
                self.message = Some(format!("color set to {}", color));
            } else {
                self.message = Some(format!(
                    "unknown color '{}' — valid: {}",
                    color,
                    COLORS.join(", ")
                ));
            }
            return false;
        }

        if input.starts_with('/') {
            handle_cmd(&input, &mut self.history, &mut self.utility);
        } else {
            let timestamp = Local::now().format("%H:%M:%S").to_string();
            self.output.push(format!("[{}] {}", timestamp, input));
            self.history.push((timestamp.clone(), input.clone()));
            save_line(&self.session, &timestamp, &input);
        }
        false
    }

    pub fn insert_char(&mut self, c: char)
    {
        self.message = None;
        let byte_pos = char_to_byte(&self.input, self.cursor_pos);
        self.input.insert(byte_pos, c);
        self.cursor_pos += 1;
        self.update_suggestion();
    }

    pub fn delete_char(&mut self)
    {
        self.message = None;
        if self.cursor_pos > 0 {
            let byte_pos = char_to_byte(&self.input, self.cursor_pos - 1);
            self.input.remove(byte_pos);
            self.cursor_pos -= 1;
        }
        self.update_suggestion();
    }

    pub fn accept_suggestion(&mut self)
    {
        if let Some(sug) = self.suggestion.clone() {
            self.input = sug;
            self.cursor_pos = self.input.chars().count();
            self.suggestion = None;
        }
    }

    pub fn cursor_left(&mut self)
    {
        self.cursor_pos = self.cursor_pos.saturating_sub(1);
    }

    pub fn cursor_right(&mut self)
    {
        self.cursor_pos = (self.cursor_pos + 1).min(self.input.chars().count());
    }

    pub fn history_up(&mut self)
    {
        if self.nav_history.is_empty() { return; }
        let new_index = match self.history_index
        {
            None => {
                self.saved_input = self.input.clone();
                self.nav_history.len() - 1
            }
            Some(i) => i.saturating_sub(1),
        };
        self.history_index = Some(new_index);
        self.input = self.nav_history[new_index].clone();
        self.cursor_pos = self.input.chars().count();
        self.suggestion = None;
    }

    pub fn history_down(&mut self)
    {
        match self.history_index
        {
            None => {}
            Some(i) => {
                if i + 1 >= self.nav_history.len() {
                    self.history_index = None;
                    self.input = self.saved_input.clone();
                } else {
                    self.history_index = Some(i + 1);
                    self.input = self.nav_history[i + 1].clone();
                }
                self.cursor_pos = self.input.chars().count();
                self.suggestion = None;
            }
        }
    }

    pub fn color_hint(&self) -> Option<String>
    {
        if !self.input.starts_with("/color") { return None; }
        let partial = self.input.trim_start_matches("/color").trim_start_matches(' ');
        let matches: Vec<&str> = COLORS.iter()
            .filter(|c| c.starts_with(partial))
            .copied()
            .collect();
        if matches.is_empty() { None } else { Some(matches.join("  ")) }
    }

    fn update_suggestion(&mut self)
    {
        let input = &self.input;

        if input.is_empty() {
            self.suggestion = None;
            return;
        }

        if input.starts_with("/color ") {
            let partial = input.trim_start_matches("/color ");
            if let Some(color) = COLORS.iter().find(|c| c.starts_with(partial) && **c != partial) {
                self.suggestion = Some(format!("/color {}", color));
                return;
            }
        }

        if input.starts_with("/session ") {
            let partial = input.trim_start_matches("/session ");
            let sessions = list_sessions();
            if let Some(s) = sessions.iter().find(|s| s.starts_with(partial) && s.as_str() != partial) {
                self.suggestion = Some(format!("/session {}", s));
                return;
            }
        }

        if input.starts_with('/') {
            if let Some(cmd) = COMMANDS.iter().find(|c| c.starts_with(input.as_str()) && **c != input) {
                self.suggestion = Some(cmd.to_string());
                return;
            }
        }

        self.suggestion = None;
    }
}
