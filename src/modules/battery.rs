use super::{Module, ModuleConfig, ModulePosition};
use crate::styles;
use battery::{Battery, Manager, State};
use ratatui::{
    style::Style,
    text::{self, Span},
};
use std::error::Error;

pub struct BatteryModule {
    level: u8,
    charging: bool,
    config: ModuleConfig,
}

impl BatteryModule {
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            level: 0,
            charging: false,
            config,
        }
    }

    fn get_icon(&self) -> &'static str {
        if self.charging {
            return "󰂄";
        }

        match self.level {
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
}

impl Module for BatteryModule {
    fn name(&self) -> &str {
        "battery"
    }

    fn position(&self) -> ModulePosition {
        self.config.position.clone()
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        let manager = Manager::new()?;
        let mut batteries = manager.batteries()?;

        if let Some(Ok(battery)) = batteries.next() {
            self.level = (battery.state_of_charge().value * 100.0) as u8;
            self.charging = matches!(battery.state(), State::Charging | State::Full);
        }
        Ok(())
    }

    fn render(&self) -> Span {
        let format = self.config.format.as_deref().unwrap_or("{icon} {level}%");
        let icon = self.get_icon();

        let text = format
            .replace("{icon}", icon)
            .replace("{level}", &self.level.to_string());

        Span::styled(text, styles::battery_style(self.charging, self.level))
    }

    fn on_click(&mut self, x: u16, y: u16) -> Result<(), Box<dyn Error>> {
        let manager = Manager::new()?;
        let mut batteries = manager.batteries()?;

        if let Some(Ok(battery)) = batteries.next() {
            let state = match battery.state() {
                State::Charging => "Charging",
                State::Discharging => "Discharging",
                State::Full => "Charged",
                State::Empty => "Empty",
                _ => "Unknown",
            };

            let time_to_full = battery
                .time_to_full()
                .map(|t| format!("{:.0} minutes", t.get::<battery::units::time::minute>()))
                .unwrap_or_else(|| "N/A".to_string());

            let time_to_empty = battery
                .time_to_empty()
                .map(|t| format!("{:.0} minutes", t.get::<battery::units::time::minute>()))
                .unwrap_or_else(|| "N/A".to_string());

            let health = (battery.state_of_health().value * 100.0) as u8;

            let info = format!(
                "Battery: {}%\nState: {}\nHealth: {}%\nTime to full: {}\nTime to empty: {}",
                self.level, state, health, time_to_full, time_to_empty
            );

            std::process::Command::new("notify-send")
                .arg("Battery Info")
                .arg(&info)
                .arg("-t")
                .arg("5000")
                .spawn()?;
        }
        Ok(())
    }

    fn on_scroll(&mut self, delta: i32) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
