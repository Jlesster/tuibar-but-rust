use super::{Module, ModuleConfig, ModulePosition};
use crate::styles;
use ratatui::{style::Style, text::Span};
use std::error::Error;
use std::fs::{self, read_to_string};
use std::process::Command;
use sysinfo::System;

pub struct WebModule {
    connected: bool,
    interface: String,
    ssid: String,
    download_speed: f64,
    upload_speed: f64,
    prev_rx_bytes: u64,
    prev_tx_bytes: u64,
    config: ModuleConfig,
}

impl WebModule {
    pub fn new(config: ModuleConfig) -> Self {
        Self {
            connected: false,
            interface: String::from("wlan0"),
            ssid: String::new(),
            download_speed: 0.0,
            upload_speed: 0.0,
            prev_rx_bytes: 0,
            prev_tx_bytes: 0,
            config,
        }
    }

    fn get_icon(&self) -> &'static str {
        if self.connected { "󰖟 " } else { "󰖪 " }
    }

    fn check_connection(&mut self) -> Result<(), Box<dyn Error>> {
        // Gracefully handle command failure
        match Command::new("ip")
            .args(&["link", "show", &self.interface])
            .output()
        {
            Ok(output) if output.status.success() => {
                let status = String::from_utf8_lossy(&output.stdout);
                self.connected = status.contains("state UP");
            }
            _ => {
                self.connected = false;
                self.ssid.clear();
                return Ok(()); // Don't propagate error
            }
        }

        if self.connected {
            match Command::new("iwgetid").args(&["-r"]).output() {
                Ok(output) if output.status.success() => {
                    self.ssid = String::from_utf8_lossy(&output.stdout).trim().to_string();
                }
                _ => {
                    self.ssid = String::from("Unknown");
                }
            }
        } else {
            self.ssid.clear();
        }
        Ok(())
    }

    fn calculate_speeds(&mut self) -> Result<(), Box<dyn Error>> {
        let rx_path = format!("/sys/class/net/{}/statistics/rx_bytes", self.interface);
        let tx_path = format!("/sys/class/net/{}/statistics/tx_bytes", self.interface);

        // Safely read files, don't crash if they don't exist
        let rx_bytes = fs::read_to_string(&rx_path)
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())
            .unwrap_or(0);

        let tx_bytes = fs::read_to_string(&tx_path)
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())
            .unwrap_or(0);

        if self.prev_rx_bytes > 0 {
            let rx_diff = rx_bytes.saturating_sub(self.prev_rx_bytes);
            let tx_diff = tx_bytes.saturating_sub(self.prev_tx_bytes);

            self.download_speed = (rx_diff as f64) / 1024.0 / 2.0;
            self.upload_speed = (tx_diff as f64) / 1024.0 / 2.0;
        }
        self.prev_rx_bytes = rx_bytes;
        self.prev_tx_bytes = tx_bytes;

        Ok(())
    }

    fn format_speed(speed: f64) -> String {
        if speed >= 1024.0 {
            format!("{:.1} MB/s", speed / 1024.0)
        } else {
            format!("{:.0} KB/s", speed)
        }
    }
}

impl Module for WebModule {
    fn name(&self) -> &str {
        "network"
    }

    fn position(&self) -> ModulePosition {
        self.config.position.clone()
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        // Don't propagate errors - just log them
        if let Err(e) = self.check_connection() {
            eprintln!("Network check failed: {}", e);
        }
        if let Err(e) = self.calculate_speeds() {
            eprintln!("Speed calculation failed: {}", e);
        }
        Ok(())
    }

    fn render(&self) -> Span {
        let format = self.config.format.as_deref().unwrap_or("{icon} {ssid}");
        let icon = self.get_icon();

        let text = format
            .replace("{icon}", icon)
            .replace("{ssid}", &self.ssid)
            .replace("{download}", &Self::format_speed(self.download_speed))
            .replace("{upload}", &Self::format_speed(self.upload_speed))
            .replace("{interface}", &self.interface);

        Span::styled(text, styles::network_style())
    }

    fn on_click(&mut self, x: u16, y: u16) -> Result<(), Box<dyn Error>> {
        std::process::Command::new("nm-connection-editor")
            .spawn()
            .or_else(|_| {
                std::process::Command::new("kitty")
                    .arg("-e")
                    .arg("nmtui")
                    .spawn()
            })?;
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
