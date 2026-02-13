use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
struct Workspace {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Window {
    class: String,
    title: String,
}

pub struct HyprlandClient {
    instance_signature: String,
}

impl HyprlandClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let signature =
            std::env::var("HYPRLAND_INSTANCE_SIGNATURE").map_err(|_| "Not running hyprland")?;

        Ok(Self {
            instance_signature: signature,
        })
    }

    pub fn get_active_workspace(&self) -> Result<u32, Box<dyn Error>> {
        let output = Command::new("hyprctl")
            .args(["activeworkspace", "-j"])
            .output()?;

        let workspace: Workspace = serde_json::from_slice(&output.stdout)?;
        Ok(workspace.id)
    }

    pub fn get_active_window(&self) -> Result<String, Box<dyn Error>> {
        let output = Command::new("hyprctl")
            .args(["activewindow", "-j"])
            .output()?;

        let window: Window = serde_json::from_slice(&output.stdout)?;

        if !window.title.is_empty() {
            Ok(window.title)
        } else {
            Ok(window.class)
        }
    }
}
