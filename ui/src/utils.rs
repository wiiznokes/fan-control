use data::node::NodeTypeLight;
use iced::{
    widget::{self, svg::Handle, Button},
    Length,
};
use once_cell::sync::Lazy;

static RESSOURCE_PATH: &str = "./ressource/icons/";
static EXTENSION: &str = ".svg";

static mut BUF: Lazy<String> = Lazy::new(|| String::with_capacity(50));

pub fn icon_button<M>(name: &str) -> Button<M> {
    widget::Button::new(my_icon(name))
}

pub fn my_icon(name: &str) -> widget::svg::Svg {
    let handle = get_handle_icon(name);
    widget::svg::Svg::new(handle).width(Length::Shrink)
}

fn get_handle_icon(name: &str) -> Handle {
    unsafe {
        BUF.clear();
        BUF.insert_str(0, RESSOURCE_PATH);
        BUF.insert_str(BUF.len(), name);
        BUF.insert_str(BUF.len(), EXTENSION);
    };

    Handle::from_path(unsafe { BUF.as_str() })
}

pub fn icon_path_for_node_type(node_type: &NodeTypeLight) -> &'static str {
    match node_type {
        NodeTypeLight::Control => "items/speed24",
        NodeTypeLight::Fan => "items/toys_fan24",
        NodeTypeLight::Temp => "items/thermometer24",
        NodeTypeLight::CustomTemp => "items/thermostat24",
        NodeTypeLight::Graph => "items/psychology24",
        NodeTypeLight::Flat => "items/horizontal_rule24",
        NodeTypeLight::Linear => "items/linear24",
        NodeTypeLight::Target => "items/my_location24",
    }
}

pub fn expand_icon<'a, M>(expanded: bool) -> widget::button::Button<'a, M> {
    if expanded {
        icon_button("arrow/expand/expand_less24")
    } else {
        icon_button("arrow/expand/expand_more24")
    }
}
