use std::path::PathBuf;

use iced::{
    color, widget::{self, button, svg::Handle, Button}, Color, Length
};

use data::node::NodeTypeLight;
use once_cell::sync::Lazy;

lazy_static::lazy_static! {
    static ref ICONS_DIR: PathBuf = utils::resource_dir().join("icons/");
}

static EXTENSION: &str = "px.svg";

static mut BUF: Lazy<String> = Lazy::new(|| String::with_capacity(50));

pub fn icon_button<M>(name: &str) -> Button<M> {
    widget::Button::new(my_icon(name))
        .style(|_theme, status| {
            let mut style = button::Style::default();

            let grey = color!(200, 200, 200);
            match status {
                button::Status::Active => { },
                button::Status::Hovered => { 
                    style = style.with_background(grey)
                },
                button::Status::Pressed => {
                    style = style.with_background(grey)
                },
                button::Status::Disabled => { },
            };
            style
        })
}

static ICON_LENGHT: Length = Length::Fixed(25.0);

pub fn my_icon(name: &str) -> widget::svg::Svg {
    let handle = get_handle_icon(name);
    widget::svg::Svg::new(handle)
        .width(ICON_LENGHT)
        .height(ICON_LENGHT)
}

fn get_handle_icon(name: &str) -> Handle {
    unsafe {
        BUF.clear();
        BUF.insert_str(0, name);
        BUF.insert_str(BUF.chars().count(), EXTENSION);
    };

    let path = ICONS_DIR.join(unsafe { BUF.as_str() });
    Handle::from_path(path)
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

pub fn expand_icon<'a, M>(expanded: bool) -> widget::button::Button<'a, M> {
    if expanded {
        icon_button("expand_less/24")
    } else {
        icon_button("expand_more/24")
    }
}
