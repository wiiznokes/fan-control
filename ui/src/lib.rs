use std::{collections::HashMap, path::PathBuf, time::Duration};

use data::{
    AppState,
    config::Config,
    node::{IsValid, NodeType, validate_name},
    settings::AppTheme,
    utils::RemoveElem,
};
use drawer::{Drawer, about};
use graph::GraphWindow;
use hardware::{HardwareBridge, Mode};
use item::items_view;
use message::{ModifNodeMsg, SettingsMsg, ToogleMsg};
use node_cache::{NodeC, NodesC};

use crate::{
    drawer::settings_drawer, graph::graph_window_view, message::NavBarContextMenuMsg,
    tray::SystemTrayMsg,
};

use cosmic::{
    ApplicationExt, Apply, Element,
    app::{
        Core, CosmicFlags, Task,
        context_drawer::{ContextDrawer, context_drawer},
    },
    executor,
    iced::{self, Subscription, time, window},
    iced_core::Length,
    iced_runtime::Action,
    theme,
    widget::{
        Column, Row, Space, menu, nav_bar, scrollable, text,
        toaster::{self, Toast, Toasts},
    },
};

use crate::message::{AppMsg, ControlMsg, CustomTempMsg, FlatMsg, LinearMsg, TargetMsg};

use crate::add_node::add_node_button_view;
use crate::config_dialogs::{
    CreateConfigDialog, CreateConfigDialogMsg, RenameConfigDialog, RenameConfigDialogMsg,
};
use crate::udev_dialog::UdevDialogMsg;

use common::{APP, ORG, QUALIFIER};
use directories::ProjectDirs;
use fslock::LockFile;

#[macro_use]
extern crate log;

#[macro_use]
pub mod localize;

mod add_node;
mod config_dialogs;
mod drawer;
mod graph;
mod headers;
mod icon;
mod input_line;
mod item;
mod message;
mod my_widgets;
mod node_cache;
mod pick_list_utils;
mod start_at_login;
mod tray;
mod udev_dialog;
mod utils;

impl<H: HardwareBridge> CosmicFlags for Flags<H> {
    type SubCommand = String;

    type Args = Vec<String>;
}

pub fn run_ui<H: HardwareBridge + 'static>(mut app_state: AppState<H>) {
    // ensure single instance
    let instance_lock_path = if cfg!(debug_assertions) {
        let _ = std::fs::create_dir_all("temp");
        PathBuf::from("temp").join("app.lock")
    } else {
        let project_dirs = ProjectDirs::from(QUALIFIER, ORG, APP).unwrap();
        project_dirs.cache_dir().join("app.lock")
    };
    let mut app_lock = LockFile::open(&instance_lock_path).expect("Failed to open app lock file");
    if !app_lock.try_lock_with_pid().unwrap_or(false) {
        info!(
            "Another instance is already running. PID can be found in {:?}",
            instance_lock_path
        );
        if let Err(e) = app_state.bridge.shutdown() {
            error!("shutdown hardware: {e}");
        }
        return;
    }

    utils::setup_wgpu();

    let settings = cosmic::app::Settings::default()
        .theme(to_cosmic_theme(&app_state.dir_manager.settings().theme))
        .no_main_window(true);

    let flags = Flags { app_state };

    if let Err(e) = cosmic::app::run::<Ui<H>>(settings, flags) {
        error!("error while running ui: {e}");
        panic!()
    }
}

struct Flags<H: HardwareBridge> {
    app_state: AppState<H>,
}

enum NavModelData {
    NoConfig,
    Config(String),
    NewConfig,
}

struct Ui<H: HardwareBridge> {
    core: Core,
    app_state: AppState<H>,
    create_button_expanded: bool,
    nodes_c: NodesC,
    graph_window: Option<GraphWindow>,
    toasts: Toasts<AppMsg>,
    dialog: Option<Dialog>,
    drawer: Option<Drawer>,
    nav_bar_model: nav_bar::Model,
    tray: Option<(tray::SystemTray, tray::SystemTrayStream)>,
    main_window: Option<window::Id>,
}

impl<H: HardwareBridge> Ui<H> {
    fn open_main_window(&mut self) -> Task<AppMsg> {
        let mut commands = Vec::new();
        let settings = window::Settings {
            size: iced::Size::new(1500.0, 800.0),
            decorations: false,
            ..Default::default()
        };

        let (window_id, command) = cosmic::iced::window::open(settings);

        commands.push(command.map(|_| cosmic::action::Action::None));

        self.main_window = Some(window_id);

        self.core.set_main_window_id(Some(window_id));

        Task::batch(commands)
    }

    fn on_exit(&mut self) {
        if let Err(e) = self.app_state.bridge.shutdown() {
            error!("shutdown hardware: {e}");
        }

        let runtime_config = Config::from_app_graph(&self.app_state.app_graph);

        if match self.app_state.dir_manager.get_config() {
            Some(saved_config) => saved_config != runtime_config,
            None => true,
        } {
            if let Err(err) = self
                .app_state
                .dir_manager
                .save_config_cached(&runtime_config)
            {
                error!("{err}")
            } else {
                info!("cached config saved successfully");
            }
        } else if let Err(err) = self.app_state.dir_manager.remove_config_cached() {
            error!("{err}")
        }
    }
}

impl<H: HardwareBridge + 'static> cosmic::Application for Ui<H> {
    type Executor = executor::Default;
    type Message = AppMsg;
    type Flags = Flags<H>;

    const APP_ID: &'static str = common::APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app_state = flags.app_state;

        let dialog = if cfg!(FAN_CONTROL_FORMAT = "flatpak")
            && app_state.dir_manager.state().show_flatpak_dialog
        {
            Some(Dialog::Udev)
        } else {
            None
        };

        let tray = match tray::SystemTray::new() {
            Ok(tray) => Some(tray),
            Err(e) => {
                error!("can't create tray {e}");
                None
            }
        };

        let mut ui_state = Ui {
            nodes_c: NodesC::new(app_state.app_graph.nodes.values()),
            app_state,
            core,
            create_button_expanded: false,
            graph_window: None,
            toasts: Toasts::new(AppMsg::RemoveToast),
            dialog,
            drawer: None,
            nav_bar_model: nav_bar::Model::default(),
            tray,
            main_window: None,
        };

        ui_state.reload_nav_bar_model();
        ui_state.update_tray_state();

        let mut commands = vec![];
        commands.push(cosmic::task::message(AppMsg::Tick));

        if !ui_state.app_state.dir_manager.settings().start_minimized {
            commands.push(ui_state.open_main_window());
        }

        (ui_state, Task::batch(commands))
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        dbg!(&message);

        let dir_manager = &mut self.app_state.dir_manager;

        match message {
            AppMsg::Tick => {
                self.update_hardware();
            }
            AppMsg::ModifNode(id, modif_node_msg) => {
                let node = self.app_state.app_graph.get_mut(&id);
                match modif_node_msg {
                    ModifNodeMsg::ChangeHardware(hardware_id) => {
                        let bridge = &mut self.app_state.bridge;

                        match &mut node.node_type {
                            NodeType::Control(i) => {
                                if i.is_valid()
                                    && let Err(e) = i.set_mode(Mode::Auto, bridge)
                                {
                                    error!(
                                        "Can't set control to auto when removing his hardware ref: {e}."
                                    );
                                }

                                i.hardware_id = hardware_id;
                                i.control_h = match &i.hardware_id {
                                    Some(hardware_id) => bridge
                                        .hardware()
                                        .controls
                                        .iter()
                                        .find(|h| &h.hardware_id == hardware_id)
                                        .cloned(),

                                    None => None,
                                }
                            }
                            NodeType::Fan(i) => {
                                i.hardware_id = hardware_id;
                                i.fan_h = match &i.hardware_id {
                                    Some(hardware_id) => bridge
                                        .hardware()
                                        .fans
                                        .iter()
                                        .find(|h| &h.hardware_id == hardware_id)
                                        .cloned(),

                                    None => None,
                                }
                            }
                            NodeType::Temp(i) => {
                                i.hardware_id = hardware_id;
                                i.temp_h = match &i.hardware_id {
                                    Some(hardware_id) => bridge
                                        .hardware()
                                        .temps
                                        .iter()
                                        .find(|h| &h.hardware_id == hardware_id)
                                        .cloned(),

                                    None => None,
                                }
                            }
                            _ => panic!("node have no hardware id"),
                        }
                    }
                    ModifNodeMsg::ReplaceInput(input) => {
                        node.inputs.clear();

                        if let Some(input) = &input {
                            node.inputs.push(input.clone())
                        }

                        let optional_name = match input {
                            Some(input) => Some(input.name),
                            None => None,
                        };
                        match &mut node.node_type {
                            NodeType::Control(i) => i.input = optional_name,
                            NodeType::Graph(i) => i.input = optional_name,
                            NodeType::Linear(i, ..) => i.input = optional_name,
                            NodeType::Target(i, ..) => i.input = optional_name,
                            _ => panic!("node have not exactly one input"),
                        }
                    }
                    ModifNodeMsg::AddInput(input) => {
                        node.inputs.push(input.clone());

                        match &mut node.node_type {
                            NodeType::CustomTemp(i) => i.inputs.push(input.name),
                            _ => panic!("node have not multiple inputs"),
                        }
                    }
                    ModifNodeMsg::RemoveInput(input) => {
                        node.inputs.remove_elem(|i| i.id == input.id);

                        match &mut node.node_type {
                            NodeType::CustomTemp(i) => {
                                i.inputs.remove_elem(|n| n == &input.name);
                            }
                            _ => panic!("node have not multiple inputs"),
                        }
                    }
                    ModifNodeMsg::Control(control_msg) => match control_msg {
                        ControlMsg::Active(is_active) => {
                            let control = node.node_type.unwrap_control_mut();
                            control.active = is_active;
                        }
                    },
                    ModifNodeMsg::CustomTemp(custom_temp_msg) => match custom_temp_msg {
                        CustomTempMsg::Kind(kind) => {
                            let custom_temp = node.node_type.unwrap_custom_temp_mut();
                            custom_temp.kind = kind;
                        }
                    },
                    ModifNodeMsg::Flat(flat_msg) => match flat_msg {
                        FlatMsg::Value(value) => {
                            let flat = node.node_type.unwrap_flat_mut();
                            flat.value = value;
                            node.value = Some(value.into());
                        }
                    },
                    ModifNodeMsg::Linear(linear_msg) => {
                        let linear = node.node_type.unwrap_linear_mut();
                        let linear_c = self.nodes_c.get_mut(&id).node_type_c.unwrap_linear_mut();

                        match linear_msg {
                            LinearMsg::MinTemp(min_temp, cached_value) => {
                                linear.min_temp = min_temp;
                                linear_c.min_temp = cached_value;
                            }
                            LinearMsg::MinSpeed(min_speed, cached_value) => {
                                linear.min_speed = min_speed;
                                linear_c.min_speed = cached_value;
                            }
                            LinearMsg::MaxTemp(max_temp, cached_value) => {
                                linear.max_temp = max_temp;
                                linear_c.max_temp = cached_value;
                            }
                            LinearMsg::MaxSpeed(max_speed, cached_value) => {
                                linear.max_speed = max_speed;
                                linear_c.max_speed = cached_value;
                            }
                        }
                    }
                    ModifNodeMsg::Target(target_msg) => {
                        let target = node.node_type.unwrap_target_mut();
                        let target_c = self.nodes_c.get_mut(&id).node_type_c.unwrap_target_mut();

                        match target_msg {
                            TargetMsg::IdleTemp(idle_temp, cached_value) => {
                                target.idle_temp = idle_temp;
                                target_c.idle_temp = cached_value;
                            }
                            TargetMsg::IdleSpeed(idle_speed, cached_value) => {
                                target.idle_speed = idle_speed;
                                target_c.idle_speed = cached_value;
                            }
                            TargetMsg::LoadTemp(load_temp, cached_value) => {
                                target.load_temp = load_temp;
                                target_c.load_temp = cached_value;
                            }
                            TargetMsg::LoadSpeed(load_speed, cached_value) => {
                                target.load_speed = load_speed;
                                target_c.load_speed = cached_value;
                            }
                        }
                    }
                    ModifNodeMsg::Delete => {
                        match self.app_state.app_graph.remove_node(id) {
                            Some(mut node) => {
                                if let NodeType::Control(control) = &mut node.node_type
                                    && let Err(e) =
                                        control.set_mode(Mode::Auto, &mut self.app_state.bridge)
                                {
                                    error!("can't set unactive when removing a control: {e}");
                                }
                            }
                            None => error!("Node was not found when trying to remove it"),
                        }

                        self.nodes_c.remove(&id);
                        self.app_state.app_graph.sanitize_inputs(false)
                    }
                    ModifNodeMsg::Graph(graph_msg) => {
                        let graph = node.node_type.unwrap_graph_mut();
                        let _graph_c = self.nodes_c.get_mut(&id).node_type_c.unwrap_graph_mut();

                        match graph_msg {
                            message::GraphMsg::RemoveCoord(coord) => {
                                graph.remove_coord(&coord);
                            }
                            message::GraphMsg::AddCoord(coord) => {
                                graph.add_coord(coord);
                            }
                            message::GraphMsg::ReplaceCoord { previous, new } => {
                                graph.replace_coord(&previous, new);
                            }
                        }
                    }
                }

                self.app_state.update.set_invalid_root_nodes_to_auto(
                    &mut self.app_state.app_graph.nodes,
                    &self.app_state.app_graph.root_nodes,
                    &mut self.app_state.bridge,
                );
            }
            AppMsg::Settings(settings_msg) => match settings_msg {
                SettingsMsg::Theme(theme) => {
                    dir_manager.update_settings(|settings| {
                        settings.theme = theme;
                    });

                    return cosmic::command::set_theme(to_cosmic_theme(&theme));
                }
                SettingsMsg::UpdateDelay(update_delay) => dir_manager.update_settings(|settings| {
                    settings.update_delay = update_delay;
                }),
                SettingsMsg::StartAtLogin(start_at_login) => {
                    start_at_login::start_at_login(start_at_login, &mut self.app_state.dir_manager);
                }
                SettingsMsg::Inactive(inactive) => self.set_inactive(inactive),
                SettingsMsg::StartMinimized(start_minimized) => {
                    dir_manager.update_settings(|settings| {
                        settings.start_minimized = start_minimized;
                    })
                }
            },
            AppMsg::NewNode(node_type_light) => {
                let node = self.app_state.app_graph.create_new_node(node_type_light);
                let node_c = NodeC::new(&node);
                self.nodes_c.insert(node.id, node_c);
                self.app_state.app_graph.insert_node(node);
            }
            AppMsg::Toggle(ui_msg) => match ui_msg {
                ToogleMsg::CreateButton(expanded) => self.create_button_expanded = expanded,
                ToogleMsg::Settings => match &self.drawer {
                    Some(drawer) => match drawer {
                        Drawer::Settings => {
                            self.drawer = None;
                            self.set_show_context(false);
                        }
                        Drawer::About => {
                            self.drawer = Some(Drawer::Settings);
                            self.set_show_context(true);
                        }
                    },
                    None => {
                        self.drawer = Some(Drawer::Settings);
                        self.set_show_context(true);
                    }
                },
                ToogleMsg::NodeContextMenu(id, expanded) => {
                    let node_c = self.nodes_c.get_mut(&id);
                    node_c.context_menu_expanded = expanded;
                }
                ToogleMsg::About => {
                    self.drawer = Some(Drawer::About);
                    self.set_show_context(true)
                }
                ToogleMsg::CloseDrawer => {
                    self.set_show_context(false);
                    self.drawer = None;
                }
            },
            AppMsg::SaveConfig(name) => return self.save_config(&name),
            AppMsg::Rename(id, name) => {
                let name_is_valid = validate_name(&self.app_state.app_graph.nodes, &id, &name);

                let node = self.app_state.app_graph.get_mut(&id);
                let node_c = self.nodes_c.get_mut(&id);

                node_c.name.clone_from(&name);
                if name_is_valid {
                    node_c.is_error_name = false;
                    let previous_name = node.name().clone();
                    node.node_type.set_name(name.clone());

                    let node_id = node.id;

                    // find nodes that depend on node.id
                    // change the name in input and item.input
                    for n in self.app_state.app_graph.nodes.values_mut() {
                        if let Some(node_input) = n
                            .inputs
                            .iter_mut()
                            .find(|node_input| node_input.id == node_id)
                        {
                            node_input.name.clone_from(&name);
                            let mut inputs = n.node_type.get_inputs();

                            match inputs.iter().position(|n| n == &previous_name) {
                                Some(index) => {
                                    inputs[index].clone_from(&name);
                                    n.node_type.set_inputs(inputs)
                                }
                                None => {
                                    error!(
                                        "input id found in node inputs but the corresponding name was not found in item input"
                                    )
                                }
                            }
                        }
                    }
                } else {
                    node_c.is_error_name = true;
                }
            }
            AppMsg::GraphWindow(graph_window_msg) => match graph_window_msg {
                graph::GraphWindowMsg::Toogle(node_id) => match (node_id, self.graph_window.take())
                {
                    (None, None) => {}
                    (None, Some(window)) => {
                        return cosmic::iced::runtime::task::effect(Action::Window(
                            window::Action::Close(window.window_id),
                        ));
                    }
                    (Some(node_id), Some(window)) if node_id == window.node_id => {
                        return cosmic::iced::runtime::task::effect(Action::Window(
                            window::Action::Close(window.window_id),
                        ));
                    }
                    (Some(node_id), Some(window)) => {
                        let mut commands = Vec::new();

                        commands.push(cosmic::iced::runtime::task::effect(Action::Window(
                            window::Action::Close(window.window_id),
                        )));

                        let (new_id, command) =
                            cosmic::iced::runtime::window::open(graph::window_settings());

                        self.graph_window = Some(GraphWindow {
                            window_id: new_id,
                            node_id,
                            temp_c: String::new(),
                            percent_c: String::new(),
                        });

                        commands.push(command.map(|_| cosmic::action::none()));

                        return Task::batch(commands);
                    }
                    (Some(node_id), None) => {
                        let mut commands = Vec::new();

                        let (new_id, command) =
                            cosmic::iced::runtime::window::open(graph::window_settings());

                        self.graph_window = Some(GraphWindow {
                            window_id: new_id,
                            node_id,
                            temp_c: String::new(),
                            percent_c: String::new(),
                        });

                        commands.push(command.map(|_| cosmic::action::none()));

                        return Task::batch(commands);
                    }
                },
                graph::GraphWindowMsg::ChangeTemp(temp) => {
                    if let Some(graph_window) = &mut self.graph_window {
                        graph_window.temp_c = temp;
                    }
                }
                graph::GraphWindowMsg::ChangePercent(percent) => {
                    if let Some(graph_window) = &mut self.graph_window {
                        graph_window.percent_c = percent;
                    }
                }
            },
            AppMsg::RemoveToast(pos) => {
                self.toasts.remove(pos);
            }
            AppMsg::Dialog(dialog_msg) => {
                return match dialog_msg {
                    DialogMsg::Udev(message) => udev_dialog::update(self, message),
                    DialogMsg::CreateConfig(create_config_dialog_msg) => {
                        CreateConfigDialog::update(self, create_config_dialog_msg)
                    }
                    DialogMsg::RenameConfig(rename_config_dialog_msg) => {
                        RenameConfigDialog::update(self, rename_config_dialog_msg)
                    }
                }
                .map(cosmic::action::app);
            }
            AppMsg::OpenUrl(url) => {
                if let Err(e) = open::that(url.as_str()) {
                    error!("{e}");
                }
            }
            AppMsg::NavBarContextMenu(nav_bar_context_menu_msg) => match nav_bar_context_menu_msg {
                NavBarContextMenuMsg::Delete(id) => {
                    if let Some(NavModelData::Config(name)) =
                        self.nav_bar_model.data::<NavModelData>(id)
                    {
                        match dir_manager.remove_config(name) {
                            Ok(is_current_config) => if is_current_config {},
                            Err(e) => {
                                error!("can't delete config: {e}");
                            }
                        }
                    }

                    self.reload_nav_bar_model();
                }
                NavBarContextMenuMsg::Rename(id) => {
                    if let Some(NavModelData::Config(name)) =
                        self.nav_bar_model.data::<NavModelData>(id)
                    {
                        self.dialog = Some(Dialog::RenameConfig(RenameConfigDialog::new(name)))
                    }
                }
            },
            AppMsg::SystemTray(msg) => match msg {
                SystemTrayMsg::Show => {
                    if let Some(main_window) = &self.main_window {
                        // avoid duplicate window
                        return cosmic::iced_runtime::task::effect(
                            cosmic::iced::runtime::Action::Window(window::Action::GainFocus(
                                *main_window,
                            )),
                        );
                    } else {
                        return self.open_main_window();
                    }
                }
                SystemTrayMsg::Config(name) => {
                    self.change_config(Some(name));
                }
                SystemTrayMsg::Inactive => {
                    self.set_inactive(!self.app_state.dir_manager.settings().inactive);
                }
                SystemTrayMsg::Exit => {
                    self.on_exit();
                    return cosmic::iced_runtime::task::effect(cosmic::iced::runtime::Action::Exit);
                }
            },
            AppMsg::HideWindow => {
                if let Some(window) = self.main_window.take() {
                    self.main_window = None;
                    self.core.set_main_window_id(None);
                    return cosmic::iced::runtime::task::effect(Action::Window(
                        window::Action::Close(window),
                    ));
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        self.view_window(self.core.main_window_id().unwrap())
    }

    fn view_window(&self, id: window::Id) -> Element<'_, Self::Message> {
        if let Some(main_window) = &self.main_window
            && main_window == &id
        {
            let app_state = &self.app_state;
            let app_graph = &app_state.app_graph;

            let content = items_view(
                &app_graph.nodes,
                &self.nodes_c,
                app_state.bridge.hardware(),
                app_state.dir_manager.settings(),
            );

            let floating_button = Column::new()
                .push(Space::new(0.0, Length::Fill))
                .push(add_node_button_view(self.create_button_expanded));

            let app = Row::new().push(content).push(floating_button);

            return toaster::toaster(&self.toasts, app);
        }

        if let Some(graph_window) = &self.graph_window
            && graph_window.window_id == id
        {
            let graph = self
                .app_state
                .app_graph
                .get(&graph_window.node_id)
                .node_type
                .unwrap_graph_ref();

            return graph_window_view(graph_window, graph);
        }

        text(format!("no view for window {id:?}")).into()
    }

    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        headers::header_start()
    }

    fn header_end(&self) -> Vec<Element<'_, Self::Message>> {
        headers::header_end(&self.app_state)
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav_bar_model)
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
        if let Some(data) = self.nav_bar_model.data::<NavModelData>(id) {
            match data {
                NavModelData::NoConfig => {
                    self.change_config(None);
                }
                NavModelData::Config(config) => {
                    self.change_config(Some(config.to_owned()));
                }
                NavModelData::NewConfig => {
                    self.dialog = Some(Dialog::CreateConfig(CreateConfigDialog::new()));
                }
            }
        }

        Task::none()
    }

    fn nav_context_menu(
        &self,
        id: nav_bar::Id,
    ) -> Option<Vec<menu::Tree<cosmic::Action<Self::Message>>>> {
        let mut items = Vec::new();

        if let Some(data) = self.nav_bar_model.data::<NavModelData>(id) {
            match data {
                NavModelData::NoConfig => {}
                NavModelData::Config(_) => {
                    items.push(cosmic::widget::menu::Item::Button(
                        fl!("rename_config"),
                        None,
                        NavBarContextMenuMsg::Rename(id),
                    ));

                    items.push(cosmic::widget::menu::Item::Button(
                        fl!("delete_config"),
                        None,
                        NavBarContextMenuMsg::Delete(id),
                    ));
                }
                NavModelData::NewConfig => {}
            }
        }

        Some(cosmic::widget::menu::items(&HashMap::new(), items))
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, Self::Message>> {
        self.drawer.as_ref().map(|drawer| match drawer {
            Drawer::Settings => context_drawer(
                settings_drawer(&self.app_state.dir_manager),
                AppMsg::Toggle(ToogleMsg::CloseDrawer),
            )
            .title(fl!("settings")),
            Drawer::About => {
                context_drawer(about(), AppMsg::Toggle(ToogleMsg::CloseDrawer)).title(fl!("about"))
            }
        })
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let mut subscriptions = vec![];

        subscriptions.push(
            time::every(Duration::from_millis(
                self.app_state.dir_manager.settings().update_delay,
            ))
            .map(|_| AppMsg::Tick),
        );

        if let Some(tray) = &self.tray {
            subscriptions.push(
                Subscription::run_with_id("system-tray", tray.1.clone().sub())
                    .map(AppMsg::SystemTray),
            );
        }

        Subscription::batch(subscriptions)

        //cosmic::iced_futures::Subscription::none()
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Self::Message> {
        println!("on_close_requested {id:?}");

        if let Some(window) = &self.main_window
            && window == &id
        {
            return Some(AppMsg::HideWindow);
        }

        if let Some(window) = &self.graph_window
            && window.window_id == id
        {
            return Some(AppMsg::GraphWindow(graph::GraphWindowMsg::Toogle(None)));
        }

        None
    }

    fn dialog(&self) -> Option<Element<'_, Self::Message>> {
        self.dialog.as_ref().map(|dialog| {
            scrollable(match dialog {
                Dialog::Udev => udev_dialog::view(),
                Dialog::CreateConfig(dialog) => dialog.view(&self.app_state.dir_manager),
                Dialog::RenameConfig(dialog) => dialog.view(&self.app_state.dir_manager),
            })
            .apply(Element::from)
            .map(AppMsg::Dialog)
        })
    }
}

#[derive(Debug)]
enum Dialog {
    Udev,
    CreateConfig(CreateConfigDialog),
    RenameConfig(RenameConfigDialog),
}

#[derive(Clone, Debug)]
enum DialogMsg {
    Udev(UdevDialogMsg),
    CreateConfig(CreateConfigDialogMsg),
    RenameConfig(RenameConfigDialogMsg),
}

impl<H: HardwareBridge> Ui<H> {
    fn update_tray_state(&self) {
        if let Some((tray, _)) = &self.tray {
            let dir_manager = &self.app_state.dir_manager;

            if let Err(e) = tray.update_menu_state(
                &dir_manager.config_names.data,
                &dir_manager.settings().current_config,
                dir_manager.settings().inactive,
            ) {
                error!("can't update tray icon: {e}");
            }
        }
    }

    fn create_config(&mut self, new_name: String) {
        let config = Config::from_app_graph(&self.app_state.app_graph);

        if let Err(e) = self.app_state.dir_manager.create_config(&new_name, &config) {
            error!("can't create config: {e}");
        }

        self.reload_nav_bar_model();
        self.update_tray_state();
    }

    fn rename_config(&mut self, prev: &str, new: &str) {
        if let Err(e) = self.app_state.dir_manager.rename_config(prev, new) {
            error!("can't rename config: {e}");
        }

        self.reload_nav_bar_model();
        self.update_tray_state();
    }

    fn change_config(&mut self, selected: Option<String>) {
        if selected.is_some() {
            self.app_state.update.set_valid_root_nodes_to_auto(
                &mut self.app_state.app_graph.nodes,
                &self.app_state.app_graph.root_nodes,
                &mut self.app_state.bridge,
            );
        }

        match self.app_state.dir_manager.change_config(selected) {
            Ok(config) => {
                if let Some((_, config)) = config {
                    self.app_state
                        .app_graph
                        .apply_config(config, self.app_state.bridge.hardware());
                    self.nodes_c = NodesC::new(self.app_state.app_graph.nodes.values());

                    self.update_hardware();
                }
            }
            Err(e) => {
                error!("can't change config: {e}");
            }
        }

        self.reload_nav_bar_model();
        self.update_tray_state();
    }

    fn save_config(&mut self, name: &str) -> Task<AppMsg> {
        let config = Config::from_app_graph(&self.app_state.app_graph);

        if let Err(e) = self.app_state.dir_manager.save_config(name, &config) {
            error!("can't save config: {e}");
            Task::none()
        } else {
            self.toasts
                .push(Toast::new(fl!("config_saved")))
                .map(cosmic::action::app)
        }
    }

    fn reload_nav_bar_model(&mut self) {
        self.nav_bar_model.clear();

        self.nav_bar_model
            .insert()
            .text(fl!("no_config"))
            .data(NavModelData::NoConfig);

        for (index, config) in self
            .app_state
            .dir_manager
            .config_names
            .names()
            .iter()
            .enumerate()
        {
            self.nav_bar_model
                .insert()
                .text(config.clone())
                .data(NavModelData::Config(config.clone()))
                .divider_above(index == 0);
        }

        match &self.app_state.dir_manager.settings().current_config {
            Some(name) => {
                if let Some(index) = self.app_state.dir_manager.config_names.index_of(name) {
                    self.nav_bar_model.activate_position((index + 1) as u16);
                }
            }
            None => {
                self.nav_bar_model.activate_position(0);
            }
        }

        self.nav_bar_model
            .insert()
            .text(fl!("create_config"))
            .icon(icon!("add/24"))
            .data(NavModelData::NewConfig)
            .divider_above(true);
    }
}

// todo: re enable when is "work" on flatpak
// currently, light theme will be the dark one
// fn to_cosmic_theme(theme: &AppTheme) -> theme::Theme {
//     match theme {
//         AppTheme::Dark => {
//             let mut t = theme::system_dark();
//             t.theme_type.prefer_dark(Some(true));
//             t
//         }
//         AppTheme::Light => {
//             let mut t = theme::system_light();
//             t.theme_type.prefer_dark(Some(false));
//             t
//         }
//         AppTheme::System => theme::system_preference(),
//     }
// }

fn to_cosmic_theme(theme: &AppTheme) -> theme::Theme {
    match theme {
        AppTheme::Dark => theme::Theme::dark(),
        AppTheme::Light => theme::Theme::light(),
        AppTheme::HighContrastDark => theme::Theme::dark_hc(),
        AppTheme::HighContrastLight => theme::Theme::light_hc(),
        AppTheme::System => theme::system_preference(),
    }
}

impl<H: HardwareBridge> Ui<H> {
    fn update_hardware(&mut self) {
        if let Err(e) = self.app_state.bridge.update() {
            error!("{e}");
            return;
        }
        if let Err(e) = self.app_state.update.all(
            &mut self.app_state.app_graph.nodes,
            &mut self.app_state.bridge,
            self.app_state.dir_manager.settings().inactive,
        ) {
            error!("{e}");
            return;
        }

        if let Err(e) = self.app_state.update.nodes_which_update_can_change(
            &mut self.app_state.app_graph.nodes,
            &mut self.app_state.bridge,
        ) {
            error!("{e}");
        }
    }

    fn set_inactive(&mut self, inactive: bool) {
        self.app_state.dir_manager.update_settings(|settings| {
            settings.inactive = inactive;
        });

        if inactive {
            self.app_state.update.set_valid_root_nodes_to_auto(
                &mut self.app_state.app_graph.nodes,
                &self.app_state.app_graph.root_nodes,
                &mut self.app_state.bridge,
            );
        } else if let Err(e) = self.app_state.update.all(
            &mut self.app_state.app_graph.nodes,
            &mut self.app_state.bridge,
            inactive,
        ) {
            error!("{e}");
        }
        self.update_tray_state();
    }
}
