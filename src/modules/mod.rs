use ratatui::{style::Style, text::Span};
use serde::{Deserialize, Serialize};
use std::{any::Any, borrow::Borrow, error::Error};

pub mod battery;
pub mod clock;
pub mod cpu;
pub mod interweb;
pub mod memory;
pub mod window;
pub mod workspaces;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleConfig {
    pub enabled: bool,
    pub format: Option<String>,
    pub interval: Option<u64>,
    pub position: ModulePosition,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum ModulePosition {
    Left,
    CenterLeft,
    Center,
    CenterRight,
    Right,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            format: None,
            interval: None,
            position: ModulePosition::Right,
        }
    }
}

pub trait Module: Send {
    fn name(&self) -> &str;
    fn position(&self) -> ModulePosition;
    fn update(&mut self) -> Result<(), Box<dyn Error>>;
    fn render(&self) -> Span;
    fn on_click(&mut self, x: u16, y: u16) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn on_scroll(&mut self, delta: i32) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
