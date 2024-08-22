use std::{path::PathBuf, sync::LazyLock};

use cosmic::{
    iced_core::Length,
    widget::{self, icon::Handle, Icon, IconButton},
};
use data::node::NodeTypeLight;

static ICONS_DIR: LazyLock<PathBuf> = LazyLock::new(|| utils::resource_dir().join("icons/"));

static EXTENSION: &str = "px.svg";

pub fn icon_button<M>(name: &str) -> widget::button::IconButton<M> {
    cosmic::widget::button::icon(get_handle_icon(name))
}

static ICON_LENGTH: Length = Length::Fixed(25.0);

pub fn my_icon(name: &str) -> Icon {
    widget::icon::icon(get_handle_icon(name))
        .height(ICON_LENGTH)
        .width(ICON_LENGTH)
}

fn get_handle_icon(name: &str) -> Handle {
    let path = ICONS_DIR.join(format!("{name}{EXTENSION}"));
    cosmic::widget::icon::from_path(path)
}

pub fn icon_path_for_node_type(node_type: &NodeTypeLight) -> &'static str {
    match node_type {
        NodeTypeLight::Control => "speed/24",
        NodeTypeLight::Fan => "toys_fan/24",
        NodeTypeLight::Temp => "thermometer/24",
        NodeTypeLight::CustomTemp => "thermostat/24",
        NodeTypeLight::Graph => "psychology/24",
        NodeTypeLight::Flat => "horizontal_rule/24",
        NodeTypeLight::Linear => "linear/24",
        NodeTypeLight::Target => "my_location/24",
    }
}

pub fn expand_icon<'a, M>(expanded: bool) -> IconButton<'a, M> {
    if expanded {
        icon_button("expand_less/24")
    } else {
        icon_button("expand_more/24")
    }
}
