use chrono::Local;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

use crate::hyprland::{self, HyprlandClient};
use crate::hyprland_ipc::{HyprlandEvent, HyprlandIPC};
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
    event_rx: Option<mpsc::UnboundedReceiver<HyprlandEvent>>,
}

impl App {
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let hyprland = HyprlandClient::new().ok();

        let initial_workspace = if let Some(ref client) = hyprland {
            client.get_active_workspace().unwrap_or(1)
        } else {
            1
        };

        if let Ok(ipc) = HyprlandIPC::new() {
            tokio::spawn(async move {
                let result = ipc
                    .listen(move |event| {
                        if let Err(e) = event_tx.send(event) {
                            eprintln!("App: Failed to send event to app: {}", e);
                        }
                    })
                    .await;

                if let Err(e) = result {
                    eprintln!("App: IPC listener error: {}", e);
                }
            });
        } else {
            eprintln!("App: #![warn(- Faled to init HyprlandIPC)]");
        }

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
            event_rx: Some(event_rx),
        }
    }

    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.process_events();
        //update time
        self.curr_time = Local::now().format("%H:%M:%S | %a %d %b").to_string();

        //Update system info
        self.update_system_info()?;
        self.update_battery()?;

        Ok(())
    }

    fn process_events(&mut self) {
        if let Some(ref mut rx) = self.event_rx {
            let mut event_count = 0;

            while let Ok(event) = rx.try_recv() {
                event_count += 1;
                match event {
                    HyprlandEvent::WorkspaceChanged(id) => {
                        self.active_workspace = id;
                    }
                    HyprlandEvent::ActiveWindowChanged(title) => {
                        self.window_title = title;
                    }
                    HyprlandEvent::Fullscreen(is_full) => {
                        // TODO: make bar  hide in FS
                    }
                    HyprlandEvent::MonitorFocused(monitor) => {
                        // TODO: make monitor hooks
                    }
                }
            }
            if event_count > 0 {
                eprintln!("App: Processed {} events this update", event_count);
            }
        } else {
            eprintln!("App: WARN - no event reciever avaliable");
        }
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
