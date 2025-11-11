use cosmic::iced::{
    futures::{SinkExt, Stream},
    stream,
};
use std::sync::{Arc, LazyLock};
use tokio::sync::{Mutex, mpsc};
use tray_icon::menu::MenuId;

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
}

#[cfg(target_os = "linux")]
pub struct SystemTray;

static MENU_ID_SHOW: LazyLock<MenuId> = LazyLock::new(|| MenuId::new("MENU_ID_SHOW"));
static MENU_ID_INACTIVE: LazyLock<MenuId> = LazyLock::new(|| MenuId::new("MENU_ID_INACTIVE"));
static MENU_ID_EXIT: LazyLock<MenuId> = LazyLock::new(|| MenuId::new("MENU_ID_EXIT"));

const MENU_ID_CONFIG_PREFIX: &str = "MENU_ID_CONFIG-";

fn menu_id_for_config(name: &str) -> MenuId {
    MenuId::new(format!("{MENU_ID_CONFIG_PREFIX}{name}"))
}

impl SystemTray {
    #[cfg(not(target_os = "linux"))]
    pub fn new() -> anyhow::Result<(Self, SystemTrayStream)> {
        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("fan-control")
            .with_icon(tray_icon()?)
            .build()?;

        let (sender, receiver) = mpsc::unbounded_channel();

        let menu_sender = sender.clone();
        MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
            // println!("{event:?}, menu inactive: {item_inactive_id:?}");
            let _ = match event.id {
                id if id == *MENU_ID_SHOW => menu_sender.send(SystemTrayMsg::Show),
                id if id == *MENU_ID_INACTIVE => menu_sender.send(SystemTrayMsg::Inactive),
                id if id == *MENU_ID_EXIT => menu_sender.send(SystemTrayMsg::Exit),
                id => {
                    if let Some(name) = id.0.strip_prefix(MENU_ID_CONFIG_PREFIX) {
                        menu_sender.send(SystemTrayMsg::Config(name.to_string()))
                    } else {
                        return;
                    }
                }
            };
        }));

        let tray_sender = sender;
        TrayIconEvent::set_event_handler(Some(move |event: TrayIconEvent| {
            if let TrayIconEvent::DoubleClick { .. } = event {
                let _ = tray_sender.send(SystemTrayMsg::Show);
            }
        }));

        Ok((
            Self { tray_icon },
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
        let menu = Menu::new();

        let item_show =
            MenuItem::with_id(MENU_ID_SHOW.clone(), fl!("tray_show_window"), true, None);

        menu.append(&item_show)?;

        let item_separator = PredefinedMenuItem::separator();
        menu.append(&item_separator)?;

        for config in configs {
            let is_active = active_config.as_ref().is_some_and(|c| c == config);

            let item_config = MenuItem::with_id(
                menu_id_for_config(config),
                if is_active {
                    format!("● {config}")
                } else {
                    format!("○ {config}")
                },
                true,
                None,
            );
            menu.append(&item_config)?;
        }

        let item_inactive = MenuItem::with_id(
            MENU_ID_INACTIVE.clone(),
            if inactive {
                format!("● {}", fl!("inactive"))
            } else {
                format!("○ {}", fl!("inactive"))
            },
            true,
            None,
        );

        menu.append(&item_inactive)?;
        menu.append(&item_separator)?;

        let item_exit = MenuItem::with_id(MENU_ID_EXIT.clone(), fl!("tray_exit"), true, None);

        menu.append(&item_exit)?;

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
