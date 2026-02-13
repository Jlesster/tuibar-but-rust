use chrono::Local;
use std::error::Error;

use crate::hyprland::{self, HyprlandClient};
use crate::system::SystemInfo;

pub struct App {
    pub curr_time: String,
    pub cpu_usage: f64,
    pub disk_usage: f64,
    pub memory_usage: f64,

    pub network_name: String,
    pub network_connected: bool,

    pub active_workspace: u32,
    pub window_title: String,

    pub battery_level: u8,
    pub battery_charging: bool,
    system_info: SystemInfo,
    hyprland: Option<HyprlandClient>,
}

impl App {
    pub fn new() -> Self {
        let hyprland = HyprlandClient::new().ok();

        Self {
            curr_time: String::new(),
            cpu_usage: 0.0,
            disk_usage: 0.0,
            memory_usage: 0.0,
            network_name: String::from("wlan0"),
            network_connected: false,
            active_workspace: 1,
            window_title: String::new(),
            battery_level: 0,
            battery_charging: false,
            system_info: SystemInfo::new(),
            hyprland,
        }
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        //update time
        self.curr_time = Local::now().format("%H:%M:%S | %a %d %b").to_string();

        //Update system info
        self.update_system_info()?;
        self.update_battery()?;
        self.update_hyprland()?;

        Ok(())
    }

    fn update_hyprland(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(ref hyprland) = self.hyprland {
            if let Ok(workspace) = hyprland.get_active_workspace() {
                self.active_workspace = workspace;
            }
            if let Ok(title) = hyprland.get_active_window() {
                self.window_title = title;
            }
        }
        Ok(())
    }

    fn update_system_info(&mut self) -> Result<(), Box<dyn Error>> {
        self.system_info.refresh();
        self.cpu_usage = self.system_info.cpu_usage();
        self.memory_usage = self.system_info.memory_usage();
        self.disk_usage = self.system_info.disk_usage();

        Ok(())
    }

    fn update_battery(&mut self) -> Result<(), Box<dyn Error>> {
        if let Ok((level, charging)) = self.system_info.battery_info() {
            self.battery_level = level;
            self.battery_charging = charging;
        }
        Ok(())
    }
}
