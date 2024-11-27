use cosmic::{
    iced_core::Length,
    widget::{self, icon::Handle, Icon, IconButton},
};

use crate::icon_handle;

pub static ICON_LENGTH: Length = Length::Fixed(25.0);

#[macro_export]
macro_rules! icon_handle {
    ($name:literal) => {{
        let bytes = include_bytes!(concat!("../../res/icons/", $name, "px.svg"));
        cosmic::widget::icon::from_svg_bytes(bytes).symbolic(true)
    }};
}

#[macro_export]
macro_rules! icon {
    ($name:literal) => {{
        use $crate::icon::ICON_LENGTH;
        use $crate::icon_handle;

        cosmic::widget::icon::icon(icon_handle!($name))
            .height(ICON_LENGTH)
            .width(ICON_LENGTH)
    }};
}
#[macro_export]
macro_rules! icon_button {
    ($name:literal) => {{
        use $crate::icon_handle;
        cosmic::widget::button::icon(icon_handle!($name))
    }};
}

#[macro_export]
macro_rules! node_icon_handle {
    ($node_type:expr) => {{
        use $crate::icon_handle;

        match $node_type {
            NodeTypeLight::Control => icon_handle!("speed/24"),
            NodeTypeLight::Fan => icon_handle!("toys_fan/24"),
            NodeTypeLight::Temp => icon_handle!("thermometer/24"),
            NodeTypeLight::CustomTemp => icon_handle!("thermostat/24"),
            NodeTypeLight::Graph => icon_handle!("psychology/24"),
            NodeTypeLight::Flat => icon_handle!("horizontal_rule/24"),
            NodeTypeLight::Linear => icon_handle!("linear/24"),
            NodeTypeLight::Target => icon_handle!("my_location/24"),
        }
    }};
}

pub fn icon_from_handle(handle: Handle) -> Icon {
    widget::icon::icon(handle)
        .height(ICON_LENGTH)
        .width(ICON_LENGTH)
}

pub fn icon_button_from_handle<'a, M>(handle: Handle) -> IconButton<'a, M> {
    cosmic::widget::button::icon(handle)
}

pub fn expand_icon<'a, M>(expanded: bool) -> IconButton<'a, M> {
    if expanded {
        icon_button!("expand_less/24")
    } else {
        icon_button!("expand_more/24")
    }
}
