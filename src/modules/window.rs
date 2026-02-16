use super::{Module, ModuleConfig, ModulePosition};
use crate::styles;
use ratatui::{style::Style, text::Span};
use serde::{Deserialize, de};
use std::error::Error;
use std::process::{Command, Output};

#[derive(Debug, Deserialize)]
struct ActiveWindow {
    class: String,
    title: String,
}

pub struct WindowModule {
    title: String,
    class: String,
    config: ModuleConfig,
    max_length: usize,
}

impl WindowModule {
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            title: String::new(),
            class: String::new(),
            config,
            max_length: 50,
        }
    }

    pub fn set_title(&mut self, title: String) {
        if !title.is_empty() {
            self.title = title;
        } else {
            self.title.clear();
        }
    }

    fn fetch_active_window(&mut self) -> Result<(), Box<dyn Error>> {
        let output = Command::new("hyprctl")
            .args(&["activewindow", "-j"])
            .output()?;

        if output.status.success() {
            let window: ActiveWindow = serde_json::from_slice(&output.stdout)?;

            if !window.title.is_empty() || !window.class.is_empty() {
                self.title = window.title;
                self.class = window.class;
            } else {
                self.title.clear();
                self.class.clear();
            }
        }
        Ok(())
    }

    fn truncate_title(&self) -> String {
        if self.title.len() > self.max_length {
            format!("{}...", &self.title[..self.max_length - 3])
        } else {
            self.title.clone()
        }
    }
}

impl Module for WindowModule {
    fn name(&self) -> &str {
        "window"
    }

    fn position(&self) -> ModulePosition {
        self.config.position.clone()
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.fetch_active_window()?;
        Ok(())
    }

    fn render(&self) -> Span {
        let format = self.config.format.as_deref().unwrap_or("{title}");

        let display_title = self.truncate_title();

        if display_title.is_empty() {
            return Span::raw("");
        }

        let text = format
            .replace("{title}", &display_title)
            .replace("{class}", &self.class);

        if text.is_empty() {
            Span::raw("")
        } else {
            Span::styled(text, Style::default().fg(crate::styles::TEXT))
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
