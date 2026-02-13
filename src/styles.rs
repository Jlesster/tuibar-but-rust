use ratatui::style::{Color, Modifier, Style};

pub const PRIMARY: Color = Color::Rgb(215, 186, 255);
pub const SURFACE: Color = Color::Rgb(22, 18, 27);
pub const TEXT: Color = Color::Rgb(233, 223, 238);
pub const PURPLE: Color = Color::Rgb(217, 189, 227);
pub const PINK: Color = Color::Rgb(234, 182, 229);
pub const GREEN: Color = Color::Rgb(181, 204, 186);
pub const YELLOW: Color = Color::Rgb(249, 226, 175);
pub const RED: Color = Color::Rgb(255, 180, 171);

pub fn workspace_style() -> Style {
    Style::default().fg(TEXT)
}

pub fn workspace_active_style() -> Style {
    Style::default()
        .fg(SURFACE)
        .bg(PRIMARY)
        .add_modifier(Modifier::BOLD)
}

pub fn clock_style() -> Style {
    Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)
}

pub fn cpu_style() -> Style {
    Style::default().fg(PURPLE)
}

pub fn memory_style() -> Style {
    Style::default().fg(PINK)
}

pub fn network_style() -> Style {
    Style::default().fg(PURPLE)
}

pub fn battery_style(charging: bool, level: u8) -> Style {
    if charging {
        Style::default().fg(GREEN)
    } else if level < 20 {
        Style::default().fg(RED)
    } else {
        Style::default().fg(TEXT)
    }
}
