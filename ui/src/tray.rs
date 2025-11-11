use cosmic::iced::{
    futures::{SinkExt, Stream},
    stream,
};
use std::sync::Arc;
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
    item_inactive: MenuItem,
}

#[cfg(target_os = "linux")]
pub struct SystemTray;

impl SystemTray {
    #[cfg(not(target_os = "linux"))]
    pub fn new() -> anyhow::Result<(Self, SystemTrayStream)> {
        let item_show = MenuItem::new(fl!("tray_show_window"), true, None);
        let item_inactive = MenuItem::new(fl!("inactive"), true, None);
        let item_exit = MenuItem::new(fl!("tray_exit"), true, None);

        let item_show_id = item_show.id().clone();
        let item_inactive_id = item_inactive.id().clone();
        let item_exit_id = item_exit.id().clone();

        let menu = Menu::with_items(&[
            &item_show,
            &PredefinedMenuItem::separator(),
            &item_inactive,
            &PredefinedMenuItem::separator(),
            &item_exit,
        ])?;

        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("fan-control")
            .with_icon(tray_icon()?)
            .build()?;

        // set up event channel
        let (sender, receiver) = mpsc::unbounded_channel();

        let menu_sender = sender.clone();
        MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
            let _ = match event.id {
                id if id == item_show_id => menu_sender.send(SystemTrayMsg::Show),
                id if id == item_inactive_id => menu_sender.send(SystemTrayMsg::Inactive),
                id if id == item_exit_id => menu_sender.send(SystemTrayMsg::Exit),
                _ => return,
            };
        }));

        let tray_sender = sender.clone();
        TrayIconEvent::set_event_handler(Some(move |event: TrayIconEvent| {
            if let TrayIconEvent::DoubleClick { .. } = event {
                let _ = tray_sender.send(SystemTrayMsg::Show);
            }
        }));

        Ok((
            Self {
                tray_icon,
                item_inactive,
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
        &mut self,
        configs: &[&str],
        active_config: Option<String>,
        inactive: bool,
    ) {
        // update menu item states
        self.item_inactive.set_enabled(inactive);
        let _ = self
            .tray_icon
            .set_tooltip(Some("fan-control".to_string()))
            .map_err(|e| {
                error!("failed to set tray icon tooltip: {e}");
            });
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
