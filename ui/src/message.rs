use cosmic::widget::ToastId;
use data::{
    config::{custom_temp::CustomTempKind, graph::Coord},
    id::Id,
    node::{Input, NodeTypeLight},
    settings::AppTheme,
};

use crate::{dialogs::DialogMsg, graph::GraphWindowMsg};

#[derive(Debug, Clone)]
pub enum AppMsg {
    Tick,
    Config(ConfigMsg),
    Settings(SettingsMsg),

    NewNode(NodeTypeLight),
    Rename(Id, String),

    Toggle(ToogleMsg),

    // can invalidate control
    ModifNode(Id, ModifNodeMsg),

    GraphWindow(GraphWindowMsg),

    RemoveToast(ToastId),
    Dialog(DialogMsg),
    OpenUrl(String),
}

#[derive(Debug, Clone)]
pub enum ConfigMsg {
    Create(String),
    Rename(String),
    Save,
    Change(Option<String>),
    Delete(String),
}

#[derive(Debug, Clone)]
pub enum SettingsMsg {
    Theme(AppTheme),
    UpdateDelay(u64),
}

#[derive(Debug, Clone)]
pub enum ToogleMsg {
    CreateButton(bool),
    Settings,
    ChooseConfig(bool),
    NodeContextMenu(Id, bool),
    About,
}

#[derive(Debug, Clone)]
pub enum ModifNodeMsg {
    Delete,
    ChangeHardware(Option<String>),
    ReplaceInput(Option<Input>),
    AddInput(Input),
    RemoveInput(Input),

    Control(ControlMsg),
    CustomTemp(CustomTempMsg),
    Flat(FlatMsg),
    Linear(LinearMsg),
    Target(TargetMsg),
    Graph(GraphMsg),
}

#[derive(Debug, Clone)]
pub enum ControlMsg {
    Active(bool),
}

#[derive(Debug, Clone)]
pub enum CustomTempMsg {
    Kind(CustomTempKind),
}

#[derive(Debug, Clone)]
pub enum FlatMsg {
    Value(u16),
}

#[derive(Debug, Clone)]
pub enum LinearMsg {
    MinTemp(u8, String),
    MinSpeed(u8, String),
    MaxTemp(u8, String),
    MaxSpeed(u8, String),
}

#[derive(Debug, Clone)]
pub enum TargetMsg {
    IdleTemp(u8, String),
    IdleSpeed(u8, String),
    LoadTemp(u8, String),
    LoadSpeed(u8, String),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum GraphMsg {
    RemoveCoord(Coord),
    AddCoord(Coord),
    #[allow(dead_code)]
    ReplaceCoord {
        previous: Coord,
        new: Coord,
    },
}

impl From<SettingsMsg> for AppMsg {
    fn from(value: SettingsMsg) -> Self {
        AppMsg::Settings(value)
    }
}

impl From<GraphWindowMsg> for AppMsg {
    fn from(value: GraphWindowMsg) -> Self {
        AppMsg::GraphWindow(value)
    }
}

impl From<ConfigMsg> for AppMsg {
    fn from(value: ConfigMsg) -> Self {
        AppMsg::Config(value)
    }
}
impl From<ToogleMsg> for AppMsg {
    fn from(value: ToogleMsg) -> Self {
        AppMsg::Toggle(value)
    }
}

impl ModifNodeMsg {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_app(self, id: Id) -> AppMsg {
        AppMsg::ModifNode(id, self)
    }
}
