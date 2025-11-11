use cosmic::iced::{
    futures::{SinkExt, Stream},
    stream,
};
use std::sync::Arc;
#[cfg(not(target_os = "linux"))]
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{Mutex, mpsc};

#[cfg(not(target_os = "linux"))]
use tray_icon::{
    TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
};

#[cfg(not(target_os = "linux"))]
use crate::fl;

#[derive(Debug, Clone)]
pub enum SystemTrayMsg {
    Show,
    Config(String),
    Inactive,
    Exit,
}

#[derive(Clone)]
pub struct SystemTrayStream {
    receiver: Arc<Mutex<mpsc::UnboundedReceiver<SystemTrayMsg>>>,
}

#[cfg(not(target_os = "linux"))]
pub struct SystemTray {
    tray_icon: TrayIcon,
    menu_sender: UnboundedSender<SystemTrayMsg>,
}

#[cfg(target_os = "linux")]
pub struct SystemTray;

impl SystemTray {
    #[cfg(not(target_os = "linux"))]
    pub fn new() -> anyhow::Result<(Self, SystemTrayStream)> {
        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("fan-control")
            .with_icon(tray_icon()?)
            .build()?;

        let (sender, receiver) = mpsc::unbounded_channel();

        let tray_sender = sender.clone();
        TrayIconEvent::set_event_handler(Some(move |event: TrayIconEvent| {
            if let TrayIconEvent::DoubleClick { .. } = event {
                let _ = tray_sender.send(SystemTrayMsg::Show);
            }
        }));

        Ok((
            Self {
                tray_icon,
                menu_sender: sender,
            },
            SystemTrayStream {
                receiver: Arc::new(Mutex::new(receiver)),
            },
        ))
    }

    #[cfg(target_os = "linux")]
    pub fn new() -> anyhow::Result<(Self, SystemTrayStream)> {
        // placeholder here
        let (_sender, receiver) = mpsc::unbounded_channel();
        Ok((
            Self {},
            SystemTrayStream {
                receiver: Arc::new(Mutex::new(receiver)),
            },
        ))
    }

    #[cfg(not(target_os = "linux"))]
    pub fn update_menu_state(
        &self,
        configs: &[String],
        active_config: &Option<String>,
        inactive: bool,
    ) -> anyhow::Result<()> {
        let item_show = MenuItem::new(fl!("tray_show_window"), true, None);
        let item_inactive = MenuItem::new(
            if inactive {
                format!("● {}", fl!("inactive"))
            } else {
                format!("○ {}", fl!("inactive"))
            },
            true,
            None,
        );
        let item_exit = MenuItem::new(fl!("tray_exit"), true, None);

        let menu = Menu::new();

        menu.append(&item_show)?;

        let item_separator = PredefinedMenuItem::separator();

        menu.append(&item_separator)?;

        let mut item_config_ids = Vec::with_capacity(configs.len());

        for config in configs {
            let is_active = active_config.as_ref().is_some_and(|c| c == config);

            let item_config = MenuItem::new(
                if is_active {
                    format!("● {config}")
                } else {
                    format!("○ {config}")
                },
                true,
                None,
            );
            item_config_ids.push((item_config.id().clone(), config.to_string()));
            menu.append(&item_config)?;
        }
        menu.append(&item_inactive)?;
        menu.append(&item_separator)?;
        menu.append(&item_exit)?;

        {
            let menu_sender = self.menu_sender.clone();
            let item_show_id = item_show.id().clone();
            let item_inactive_id = item_inactive.id().clone();
            let item_exit_id = item_exit.id().clone();

            MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
                let _ = match event.id {
                    id if id == item_show_id => menu_sender.send(SystemTrayMsg::Show),
                    id if id == item_inactive_id => menu_sender.send(SystemTrayMsg::Inactive),
                    id if id == item_exit_id => menu_sender.send(SystemTrayMsg::Exit),
                    id => {
                        if let Some((_, name)) = item_config_ids
                            .iter()
                            .find(|(config_id, _)| *config_id == id)
                        {
                            menu_sender.send(SystemTrayMsg::Config(name.clone()))
                        } else {
                            return;
                        }
                    }
                };
            }));
        }

        self.tray_icon.set_menu(Some(Box::new(menu)));

        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn update_menu_state(&mut self, _disconnected: bool, _status: &str) {
        // placeholder here
    }
}

impl SystemTrayStream {
    pub fn sub(self) -> impl Stream<Item = SystemTrayMsg> {
        let receiver_arc = self.receiver.clone();

        stream::channel(1, |mut sender| async move {
            loop {
                let mut receiver = receiver_arc.lock().await;
                if let Some(msg) = receiver.recv().await {
                    if sender.send(msg).await.is_err() {
                        break;
                    }
                } else {
                    break;
                }
            }
        })
    }
}

#[cfg(not(target_os = "linux"))]
fn tray_icon() -> Result<tray_icon::Icon, tray_icon::BadIcon> {
    let svg = include_bytes!("../../res/linux/app_icon.svg");

    let width = 32;
    let height = 32;

    let opt = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_data(svg, &opt).unwrap();
    let viewbox = tree.size();

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height).unwrap();
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(
            width as f32 / viewbox.width(),
            height as f32 / viewbox.height(),
        ),
        &mut pixmap.as_mut(),
    );

    let rgba = pixmap.data().to_vec();

    tray_icon::Icon::from_rgba(rgba, width, height)
}
