use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Styled},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::module_manager::ModuleManager;
use crate::modules::ModulePosition;
use crate::styles::*;

pub fn render_ui(f: &mut Frame, app: &App, module_manager: &ModuleManager) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(40),
            Constraint::Min(20),
            Constraint::Length(40),
        ])
        .split(size);

    let left_content = render_section(module_manager, ModulePosition::Left);
    let left = Paragraph::new(left_content).style(Style::default().bg(SURFACE));
    f.render_widget(left, chunks[0]);

    let center_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let center_content = render_section(module_manager, ModulePosition::Center);
    let center = Paragraph::new(center_content)
        .alignment(Alignment::Center)
        .style(Style::default().bg(SURFACE));
    f.render_widget(center, chunks[1]);

    let center_left_content = render_section(module_manager, ModulePosition::CenterLeft);
    let center_left = Paragraph::new(center_left_content).style(Style::default().bg(SURFACE));
    f.render_widget(center_left, center_chunks[0]);

    let center_right_content = render_section(module_manager, ModulePosition::CenterRight);
    let center_right = Paragraph::new(center_right_content)
        .alignment(Alignment::Right)
        .style(Style::default().bg(SURFACE));
    f.render_widget(center_right, center_chunks[1]);

    let right_content = render_section(module_manager, ModulePosition::Right);
    let right = Paragraph::new(right_content)
        .alignment(Alignment::Right)
        .style(Style::default().bg(SURFACE));
    f.render_widget(right, chunks[2]);
}

fn render_section(module_manager: &ModuleManager, position: ModulePosition) -> Line {
    let modules = module_manager.get_modules_for_position(position);

    let mut spans = vec![Span::raw(" ")];

    for (i, module) in modules.iter().enumerate() {
        spans.push(module.render());

        if i < modules.len() - 1 {
            spans.push(Span::raw(" | "));
        }
    }

    Line::from(spans)
}
