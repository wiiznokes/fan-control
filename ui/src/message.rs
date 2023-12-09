use data::{config::custom_temp::CustomTempKind, id::Id, node::NodeTypeLight, settings::AppTheme};

use crate::pick::Pick;

#[derive(Debug, Clone)]
pub enum AppMsg {
    Tick,

    Config(ConfigMsg),
    Settings(SettingsMsg),

    NewNode(NodeTypeLight),
    Rename(Id, String),

    Toggle(ToogleMsg),

    // require app_graph sanitizing
    ModifNode(Id, ModifNodeMsg),
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
    ChangeTheme(AppTheme),
}

#[derive(Debug, Clone)]
pub enum ToogleMsg {
    CreateButton(bool),
    Settings,
    ChooseConfig(bool),
    NodeContextMenu(Id, bool),
}

#[derive(Debug, Clone)]
pub enum ModifNodeMsg {
    Delete,
    ChangeHardware(Pick<String>),
    ReplaceInput(Pick<Id>),
    AddInput(Pick<Id>),
    RemoveInput(Pick<Id>),

    Control(ControlMsg),
    CustomTemp(CustomTempMsg),
    Flat(FlatMsg),
    Linear(LinearMsg),
    Target(TargetMsg),
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

impl From<SettingsMsg> for AppMsg {
    fn from(value: SettingsMsg) -> Self {
        AppMsg::Settings(value)
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
