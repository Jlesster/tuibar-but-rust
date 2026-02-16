use super::{Module, ModuleConfig, ModulePosition};
use crate::styles;
use ratatui::{style::Style, text::Span};
use std::error::Error;
use sysinfo::System;

pub struct CpuModule {
    usage: f64,
    system: System,
    config: ModuleConfig,
    icon: &'static str,
}

impl CpuModule {
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            usage: 0.0,
            system: System::new_all(),
            config,
            icon: "󰻠 ",
        }
    }
}

impl Module for CpuModule {
    fn name(&self) -> &str {
        "cpu"
    }

    fn position(&self) -> ModulePosition {
        self.config.position.clone()
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.system.refresh_cpu();

        let mut total = 0.0;
        let cpus = self.system.cpus();

        if !cpus.is_empty() {
            for cpu in cpus {
                total += cpu.cpu_usage() as f64;
            }
            self.usage = (total / cpus.len() as f64 * 10.0).round() / 10.0;
        }
        Ok(())
    }

    fn render(&self) -> Span {
        let format_str = self.config.format.as_deref().unwrap_or("{icon} {usage}%");

        let text = format_str
            .replace("{icon}", self.icon)
            .replace("{usage}", &format!("{:.0}", self.usage));

        Span::styled(text, styles::cpu_style())
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
