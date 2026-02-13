use battery::Manager;
use std::error::Error;
use sysinfo::System;

pub struct SystemInfo {
    sys: System,
    battery_manager: Manager,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
            battery_manager: Manager::new().unwrap_or_else(|_| {
                eprintln!("Failed to create bal manager");
                Manager::new().unwrap()
            }),
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
        self.sys.refresh_memory();
    }

    pub fn cpu_usage(&self) -> f64 {
        let mut total = 0.0;
        let cpus = self.sys.cpus();

        if cpus.is_empty() {
            return 0.0;
        }

        for cpu in cpus {
            total += cpu.cpu_usage() as f64;
        }

        (total / cpus.len() as f64 * 10.0).round() / 10.0
    }

    pub fn memory_usage(&self) -> f64 {
        let total = self.sys.total_memory() as f64;
        let used = self.sys.used_memory() as f64;

        if total == 0.0 {
            return 0.0;
        }

        ((used / total * 100.0) * 10.0).round() / 10.0
    }

    pub fn disk_usage(&self) -> f64 {
        let disks = sysinfo::Disks::new_with_refreshed_list();

        for disk in &disks {
            if disk.mount_point().to_str() == Some("/") {
                let total = disk.total_space() as f64;
                let avaliable = disk.available_space() as f64;
                let used = total - avaliable;

                if total == 0.0 {
                    return 0.0;
                }
                return ((used / total * 100.0) * 10.0).round() / 10.0;
            }
        }
        0.0
    }

    pub fn battery_info(&mut self) -> Result<(u8, bool), Box<dyn Error>> {
        let mut batteries = self.battery_manager.batteries()?;

        if let Some(maybe_battery) = batteries.next() {
            let battery = maybe_battery?;
            let percentage = (battery.state_of_charge().value * 100.0) as u8;
            let charging = matches!(
                battery.state(),
                battery::State::Charging | battery::State::Full
            );
            Ok((percentage, charging))
        } else {
            Ok((0, false))
        }
    }
}
