use ratatui::style::Color;

pub fn parse_color(s: &str) -> Color
{
    match s {
        "red"     => Color::Red,
        "green"   => Color::Green,
        "yellow"  => Color::Yellow,
        "blue"    => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan"    => Color::Cyan,
        "white"   => Color::White,
        "gray"    => Color::Gray,
        _         => Color::Cyan,
    }
}

pub fn char_to_byte(s: &str, char_idx: usize) -> usize
{
    s.char_indices().nth(char_idx).map(|(i, _)| i).unwrap_or(s.len())
}
