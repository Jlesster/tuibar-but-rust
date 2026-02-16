use super::{Module, ModuleConfig, ModulePosition};
use crate::styles;
use ratatui::{
    style::Style,
    text::{Line, Span},
};
use serde::Deserialize;
use std::process::Command;
use std::{error::Error, process::Output};

#[derive(Debug, Deserialize)]
struct Workspace {
    id: i32,
    name: String,
    windows: i32,
}

pub struct WorkspaceModule {
    active_id: u32,
    workspaces: Vec<Workspace>,
    config: ModuleConfig,
    max_workspaces: usize,
}

impl WorkspaceModule {
    pub fn new(config: ModuleConfig) -> Self {
        let mut module = Self {
            active_id: 1,
            workspaces: Vec::new(),
            config,
            max_workspaces: 7,
        };
        let _ = module.fetch_workspaces();
        let _ = module.fetch_active_workspace();

        module
    }

    pub fn set_active_workspace(&mut self, id: u32) {
        self.active_id = id.clamp(1, self.max_workspaces as u32);
    }

    fn fetch_workspaces(&mut self) -> Result<(), Box<dyn Error>> {
        let output = Command::new("hyprctl")
            .args(&["workspaces", "-j"])
            .output()?;

        if output.status.success() {
            self.workspaces = serde_json::from_slice(&output.stdout)?;
        }
        Ok(())
    }

    fn fetch_active_workspace(&mut self) -> Result<(), Box<dyn Error>> {
        let output = Command::new("hyprctl")
            .args(&["activeworkspace", "-j"])
            .output()?;

        if output.status.success() {
            let workspace: Workspace = serde_json::from_slice(&output.stdout)?;
            self.active_id = (workspace.id as u32).clamp(1, self.max_workspaces as u32);
        }
        Ok(())
    }

    fn has_windows(&self, id: i32) -> bool {
        self.workspaces
            .iter()
            .find(|w| w.id == id)
            .map(|w| w.windows > 0)
            .unwrap_or(false)
    }
}

impl Module for WorkspaceModule {
    fn name(&self) -> &str {
        "workspaces"
    }

    fn position(&self) -> ModulePosition {
        self.config.position.clone()
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.fetch_workspaces()?;
        Ok(())
    }

    fn render(&self) -> Span {
        let mut text = String::from(" ");

        for i in 1..=self.max_workspaces {
            let id = i as i32;

            let is_active = id == self.active_id as i32
                && self.active_id >= 1
                && self.active_id <= self.max_workspaces as u32;

            if id == self.active_id as i32 {
                text.push_str(&format!("[~{}~]", i));
            } else if self.has_windows(id) {
                text.push_str(&format!("[({})]", i));
            } else {
                text.push_str(&format!("[ {} ]", i));
            }
        }

        Span::styled(text, styles::workspace_style())
    }

    fn on_click(&mut self, x: u16, _y: u16) -> Result<(), Box<dyn Error>> {
        let workspace_width = 5;
        let clicked_workspace = ((x / workspace_width) + 1).min(10);

        Command::new("hyprctl")
            .args(&["dispatch", "workspace", &clicked_workspace.to_string()])
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
