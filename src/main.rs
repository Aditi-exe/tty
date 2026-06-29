mod app;
mod command;
mod constants;
mod storage;
mod utils;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;
use constants::ART;
use utils::parse_color;

fn main() -> io::Result<()>
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

    loop
    {
        terminal.draw(|frame|
        {
            let utility_height = app.utility.len().min(8) as u16;
            let art_height = if app.show_art { ART.len() as u16 + 1 } else { 0 };

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(art_height),
                    Constraint::Length(utility_height),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(frame.area());

            let color = parse_color(&app.session_color);

            // input bar
            let ghost = app.suggestion.as_deref()
                .and_then(|s| s.get(app.input.len()..))
                .unwrap_or("");
            let input_line = Line::from(vec![
                Span::raw(app.input.as_str()),
                Span::styled(ghost, Style::default().fg(Color::DarkGray)),
            ]);
            let input = Paragraph::new(input_line)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title(format!(" {} >> ", app.session))
                    .title_style(Style::default().fg(color)));
            frame.render_widget(input, chunks[0]);

            // ascii art — just below input, disappears on first keypress
            if app.show_art {
                let art_lines: Vec<Line> = ART.iter()
                    .map(|s| Line::from(Span::styled(*s, Style::default().fg(color))))
                    .collect();
                frame.render_widget(Paragraph::new(art_lines), chunks[1]);
            }

            // utility output — dimmed, no border
            if utility_height > 0 {
                let utility_lines: Vec<Line> = app.utility.iter()
                    .map(|s| Line::from(Span::styled(
                        format!("  {}", s),
                        Style::default().fg(Color::DarkGray),
                    )))
                    .collect();
                frame.render_widget(Paragraph::new(utility_lines), chunks[2]);
            }

            // main output — no border, free flowing
            app.visible_height = chunks[3].height as usize;
            let total = app.output.len();
            let start = if total > app.visible_height {
                (total - app.visible_height).saturating_sub(app.scroll as usize)
            } else {
                0
            };
            let items: Vec<ListItem> = app.output[start..]
                .iter()
                .map(|m| ListItem::new(format!("  {}", m)))
                .collect();
            frame.render_widget(List::new(items), chunks[3]);

            // status bar
            let (status_text, status_style) = if let Some(msg) = &app.message {
                (format!(" {}", msg), Style::default().fg(Color::Yellow))
            } else if let Some(hint) = app.color_hint() {
                (format!(" colors:  {}", hint), Style::default().fg(color))
            } else {
                (
                    " ctrl+c quit  |  ctrl+l clear  |  tab complete  |  ↑↓ history  |  /session <name>  |  /color <name>".to_string(),
                    Style::default().fg(Color::DarkGray),
                )
            };
            frame.render_widget(
                Paragraph::new(status_text.as_str()).style(status_style),
                chunks[4],
            );

            frame.set_cursor_position((
                chunks[0].x + app.cursor_pos as u16 + 1,
                chunks[0].y + 1,
            ));
        })?;

        match event::read()?
        {
            Event::Key(key) if key.kind == KeyEventKind::Press =>
            {
                app.show_art = false;
                match (key.code, key.modifiers)
                {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Char('l'), KeyModifiers::CONTROL) => {
                        app.output.clear();
                        app.utility.clear();
                    }
                    (KeyCode::Tab, _) => app.accept_suggestion(),
                    (KeyCode::Enter, _) => { if app.submit() { break; } }
                    (KeyCode::Backspace, _) => app.delete_char(),
                    (KeyCode::Left, _) => app.cursor_left(),
                    (KeyCode::Right, _) => app.cursor_right(),
                    (KeyCode::Up, _) => app.history_up(),
                    (KeyCode::Down, _) => app.history_down(),
                    (KeyCode::Char(c), _) => app.insert_char(c),
                    _ => {}
                }
            }
            Event::Mouse(mouse) =>
            {
                match mouse.kind
                {
                    MouseEventKind::ScrollUp => {
                        let max_scroll = app.output.len().saturating_sub(app.visible_height) as u16;
                        app.scroll = app.scroll.saturating_add(1).min(max_scroll);
                    }
                    MouseEventKind::ScrollDown => {
                        app.scroll = app.scroll.saturating_sub(1);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
