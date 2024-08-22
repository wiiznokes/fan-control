use std::{cell::LazyCell, path::PathBuf, sync::LazyLock};

use cosmic::{
    iced_core::Length,
    widget::{self, icon::Handle, Icon, IconButton},
};
use data::node::NodeTypeLight;
use once_cell::sync::Lazy;

static ICONS_DIR: LazyLock<PathBuf> = LazyLock::new(|| utils::resource_dir().join("icons/"));

static EXTENSION: &str = "px.svg";

static mut BUF: Lazy<String> = Lazy::new(|| String::with_capacity(50));

pub fn icon_button<M>(name: &str) -> widget::button::IconButton<M> {
    cosmic::widget::button::icon(get_handle_icon(name))
}

static ICON_LENGHT: Length = Length::Fixed(25.0);

pub fn my_icon(name: &str) -> Icon {
    widget::icon::icon(get_handle_icon(name))
        .height(ICON_LENGHT)
        .width(ICON_LENGHT)
}

fn get_handle_icon(name: &str) -> Handle {
    unsafe {
        BUF.clear();
        BUF.insert_str(0, name);
        BUF.insert_str(BUF.chars().count(), EXTENSION);
    };

    let path = ICONS_DIR.join(unsafe { BUF.as_str() });
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
