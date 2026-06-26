mod app;
mod command;
mod storage;

use app::App;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

fn main() -> io::Result<()>
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

    loop
    {
        terminal.draw(|frame|
        {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(frame.area());
            
            let items: Vec<ListItem> = app.output.iter()
                .map(|m| ListItem::new(m.as_str()))
                .collect();
            
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" output "));

            let input = Paragraph::new(app.input.as_str())
                .block(Block::default().borders(Borders::ALL).title(" >> "));
            
            frame.render_widget(list, chunks[0]);
            frame.render_widget(input, chunks[1]);
            frame.set_cursor_position((
                chunks[1].x + app.input.len() as u16 + 1,
                chunks[1].y + 1,
            ));
        })?;

        if let Event::Key(key) = event::read()? 
        {
            if key.kind == KeyEventKind::Press
            {
                match (key.code, key.modifiers)
                {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                    (KeyCode::Char('l'), KeyModifiers::CONTROL) => app.output.clear(),
                    (KeyCode::Enter, _) => { if app.submit() { break; } },
                    (KeyCode::Backspace, _) => { app.input.pop(); },
                    (KeyCode::Char(c), _) => app.input.push(c),
                    _ => {}
                }
            }
        } 
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}