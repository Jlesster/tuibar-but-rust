use ratatui::{style::Style, text::Span};

pub trait Module {
    fn name(&self) -> &str;
    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn render(&self) -> Span;
    fn style(&self) -> Style;
}

pub struct CpuModule {
    usage: f64,
}

impl CpuModule {
    pub fn new() -> Self {
        Self { usage: 0.0 }
    }
}

impl Module for CpuModule {
    fn name(&self) -> &str {
        "cpu"
    }

    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn style(&self) -> Style {
        crate::styles::cpu_style()
    }

    fn render(&self) -> Span {
        Span::raw(format!(" {:.1}%", self.usage))
    }
}
