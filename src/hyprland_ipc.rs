use chrono::format::parse;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixStream;

pub enum HyprlandEvent {
    WorkspaceChanged(u32),
    ActiveWindowChanged(String),
    MonitorFocused(String),
    Fullscreen(bool),
}

pub struct HyprlandIPC {
    socket_path: PathBuf,
}

impl HyprlandIPC {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let signature =
            std::env::var("HYPRLAND_INSTANCE_SIGNATURE").map_err(|_| "Not running hyprland")?;

        let socket_path = PathBuf::from(format!("/tmp/hypr/{}/.socket2.sock", signature));
        if !socket_path.exists() {
            return Err("Hyprland event socket not found".into());
        }
        Ok(Self { socket_path })
    }

    pub async fn listen<F>(&self, mut callback: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(HyprlandEvent) + Send + 'static,
    {
        let stream = UnixStream::connect(&self.socket_path).await?;
        let reader = BufReader::new(stream);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            if let Some(event) = self.parse_event(&line) {
                callback(event);
            }
        }

        Ok(())
    }
    fn parse_event(&self, line: &str) -> Option<HyprlandEvent> {
        let parts: Vec<&str> = line.splitn(2, ">>").collect();
        if parts.len() != 2 {
            return None;
        }

        let event_type = parts[0];
        let data = parts[1];

        match event_type {
            "workspace" => {
                let workspace_str = data.trim();
                if let Ok(workspace_id) = workspace_str.parse::<u32>() {
                    return Some(HyprlandEvent::WorkspaceChanged(workspace_id));
                }
                None
            }
            "activewindow" => {
                let title = data.split(',').nth(1).unwrap_or("").to_string();
                Some(HyprlandEvent::ActiveWindowChanged(title))
            }
            "focusedmon" => {
                let monitor = data.split(',').next().unwrap_or("").to_string();
                Some(HyprlandEvent::MonitorFocused(monitor))
            }
            "fullscreen" => {
                let is_fullscreen = data == "1";
                Some(HyprlandEvent::Fullscreen(is_fullscreen))
            }
            _ => None,
        }
    }
}
