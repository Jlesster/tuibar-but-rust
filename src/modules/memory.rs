use super::{Module, ModuleConfig, ModulePosition};
use crate::styles;
use ratatui::{style::Style, text::Span};
use std::error::Error;
use sysinfo::System;

pub struct MemoryModule {
    usage: f64,
    used_gb: f64,
    total_gb: f64,
    system: System,
    config: ModuleConfig,
    icon: &'static str,
}

impl MemoryModule {
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            usage: 0.0,
            used_gb: 0.0,
            total_gb: 0.0,
            system: System::new_all(),
            config,
            icon: "󰍛 ",
        }
    }
}

impl Module for MemoryModule {
    fn name(&self) -> &str {
        "memory"
    }

    fn position(&self) -> ModulePosition {
        self.config.position.clone()
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.system.refresh_memory();

        let total = self.system.total_memory() as f64;
        let used = self.system.used_memory() as f64;

        if total > 0.0 {
            self.usage = ((used / total * 100.0) * 10.0).round() / 10.0;
            self.used_gb = used / 1_073_741_824.0;
            self.total_gb = total / 1_073_741_824.0;
        }
        Ok(())
    }

    fn render(&self) -> Span {
        let format = self.config.format.as_deref().unwrap_or("{icon} {usage}%");

        let text = format
            .replace("{icon}", self.icon)
            .replace("{usage}", &format!("{:.0}", self.usage))
            .replace("{used}", &format!("{:.1}", self.used_gb))
            .replace("{total}", &format!("{:.1}", self.total_gb));

        Span::styled(text, styles::memory_style())
    }

    fn on_click(&mut self, _x: u16, _y: u16) -> Result<(), Box<dyn Error>> {
        std::process::Command::new("kitty")
            .arg("-e")
            .arg("btop")
            .spawn()?;
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
