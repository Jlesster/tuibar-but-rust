use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Styled},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::styles::*;

pub fn render_ui(f: &mut Frame, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(40),
            Constraint::Min(20),
            Constraint::Length(40),
        ])
        .split(size);
    let left_content = render_left_section(app);
    let left = Paragraph::new(left_content).style(Style::default().bg(SURFACE));
    f.render_widget(left, chunks[0]);

    let center_content = render_center_section(app);
    let center = Paragraph::new(center_content)
        .alignment(Alignment::Center)
        .style(Style::default().bg(SURFACE));
    f.render_widget(center, chunks[1]);

    let right_content = render_right_section(app);
    let right = Paragraph::new(right_content)
        .alignment(Alignment::Right)
        .style(Style::default().bg(SURFACE));
    f.render_widget(right, chunks[2]);
}

fn render_left_section(app: &App) -> Line<'static> {
    let mut spans = vec![Span::raw("  ")];

    for i in 1..=10 {
        if i == app.active_workspace {
            spans.push(Span::styled(format!(" {} ", i), workspace_active_style()));
        } else {
            spans.push(Span::styled(format!("  {}  ", i), workspace_style()));
        }
        spans.push(Span::raw(" "));
    }
    Line::from(spans)
}

fn render_center_section(app: &App) -> Line {
    let spans = vec![Span::styled(&app.curr_time, clock_style())];
    Line::from(spans)
}

fn render_right_section(app: &App) -> Line<'static> {
    let cpu_icon = "󰻠 ";
    let mem_icon = "󰍛 ";
    let disk_icon = "󰋊 ";
    let bat_icon = get_battery_icon(app.battery_level, app.battery_charging);

    let spans = vec![
        Span::styled(format!("{} {:.1}%", cpu_icon, app.cpu_usage), cpu_style()),
        Span::raw("  "),
        Span::styled(
            format!("{} {:.1}%", mem_icon, app.memory_usage),
            memory_style(),
        ),
        Span::raw("  "),
        Span::styled(
            format!("{} {}%", bat_icon, app.battery_level),
            battery_style(app.battery_charging, app.battery_level),
        ),
    ];
    Line::from(spans)
}

fn render_clock(time: &str) -> Paragraph {
    Paragraph::new(time)
        .style(clock_style())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL))
        .set_style(Style::default().fg(PRIMARY))
}

fn get_battery_icon(level: u8, charging: bool) -> &'static str {
    if charging {
        return "󰂄";
    }

    match level {
        90..=100 => "󰁹",
        80..=89 => "󰂂",
        70..=79 => "󰂁",
        60..=69 => "󰂀",
        50..=59 => "󰁿",
        40..=49 => "󰁾",
        30..=39 => "󰁽",
        20..=29 => "󰁼",
        10..=19 => "󰁻",
        _ => "󰁺",
    }
}
