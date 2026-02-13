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
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(size);

    let workspaces = render_workspaces(app.active_workspace);
    f.render_widget(workspaces, chunks[0]);

    let clock = render_clock(&app.curr_time);
    f.render_widget(clock, chunks[1]);

    let sysinfo = render_system_info(app);
    f.render_widget(sysinfo, chunks[2]);
}

fn render_workspaces(active: u32) -> Paragraph<'static> {
    let mut spans = vec![];

    for i in 1..=4 {
        if i == active {
            spans.push(Span::styled(format!(" [{}] ", i), workspace_active_style()));
        } else {
            spans.push(Span::styled(format!("  {}  ", i), workspace_style()));
        }
    }
    Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL))
        .set_style(Style::default().fg(PRIMARY))
}

fn render_clock(time: &str) -> Paragraph {
    Paragraph::new(time)
        .style(clock_style())
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL))
        .set_style(Style::default().fg(PRIMARY))
}

fn render_system_info(app: &App) -> Paragraph {
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

    Paragraph::new(Line::from(spans))
        .alignment(Alignment::Right)
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
