use super::{Module, ModuleConfig, ModulePosition};
use crate::styles;
use chrono::Local;
use ratatui::{style::Style, text::Span};
use std::error::Error;

pub struct ClockModule {
    time: String,
    config: ModuleConfig,
}

impl ClockModule {
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            time: String::new(),
            config,
        }
    }
}

impl Module for ClockModule {
    fn name(&self) -> &str {
        "clock"
    }

    fn position(&self) -> ModulePosition {
        self.config.position.clone()
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let format = self.config.format.as_deref().unwrap_or("%H:%M:%S");

        self.time = Local::now().format(format).to_string();

        Ok(())
    }

    fn render(&self) -> Span {
        Span::styled(&self.time, styles::clock_style())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
