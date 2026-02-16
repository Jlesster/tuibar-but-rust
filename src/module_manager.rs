use crate::config::Config;
use crate::hyprland_ipc::HyprlandEvent;
use crate::modules::*;
use std::error::Error;
use std::time::{Duration, Instant};

struct ModuleWithTimer {
    module: Box<dyn Module>,
    interval: Option<Duration>,
    last_update: Instant,
}

pub struct ModuleManager {
    modules: Vec<ModuleWithTimer>,
}

impl ModuleManager {
    pub fn new(config: &Config) -> Self {
        let mut modules: Vec<ModuleWithTimer> = Vec::new();

        for module_name in &config.modules {
            let module_config = config
                .module_configs
                .get(module_name)
                .cloned()
                .unwrap_or_default();

            if !module_config.enabled {
                continue;
            }

            let module: Box<dyn Module> = match module_name.as_str() {
                "cpu" => Box::new(cpu::CpuModule::new(module_config.clone())),
                "memory" => Box::new(memory::MemoryModule::new(module_config.clone())),
                "battery" => Box::new(battery::BatteryModule::new(module_config.clone())),
                "clock" => Box::new(clock::ClockModule::new(module_config.clone())),
                "workspaces" => Box::new(workspaces::WorkspaceModule::new(module_config.clone())),
                "window" => Box::new(window::WindowModule::new(module_config.clone())),
                "network" => Box::new(interweb::WebModule::new(module_config.clone())),
                _ => continue,
            };

            let interval = module_config.interval.map(Duration::from_millis);

            modules.push(ModuleWithTimer {
                module,
                interval,
                last_update: Instant::now(),
            });
        }
        Self { modules }
    }

    pub fn update_all(&mut self) -> Result<(), Box<dyn Error>> {
        let now = Instant::now();

        for module_timer in &mut self.modules {
            let should_update = match module_timer.interval {
                Some(interval) => now.duration_since(module_timer.last_update) >= interval,
                None => true,
            };

            if should_update {
                module_timer.module.update()?;
                module_timer.last_update = now;
            }
        }
        Ok(())
    }

    pub fn handle_hyprland_event(&mut self, event: &HyprlandEvent) {
        match event {
            HyprlandEvent::WorkspaceChanged(id) => {
                for module_timer in &mut self.modules {
                    if let Some(ws_module) = module_timer
                        .module
                        .as_any_mut()
                        .downcast_mut::<workspaces::WorkspaceModule>()
                    {
                        ws_module.set_active_workspace(*id);
                    }
                }
            }
            HyprlandEvent::ActiveWindowChanged(title) => {
                for module_timer in &mut self.modules {
                    if module_timer.module.name() == "window" {
                        if let Some(win_module) = module_timer
                            .module
                            .as_any_mut()
                            .downcast_mut::<window::WindowModule>()
                        {
                            win_module.set_title(title.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn get_modules_for_position(&self, pos: ModulePosition) -> Vec<&Box<dyn Module>> {
        self.modules
            .iter()
            .map(|mt| &mt.module)
            .filter(|m| m.position() == pos)
            .collect()
    }

    pub fn handle_click(&mut self, x: u16, y: u16) -> Result<(), Box<dyn Error>> {
        for module_timer in &mut self.modules {
            module_timer.module.on_click(x, y)?;
        }
        Ok(())
    }
}
