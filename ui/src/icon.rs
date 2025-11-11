use cosmic::{
    iced,
    iced_core::Length,
    widget::{self, Icon, IconButton, icon::Handle},
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

#[allow(dead_code)]
pub fn expand_icon<'a, M>(expanded: bool) -> IconButton<'a, M> {
    if expanded {
        icon_button!("expand_less/24")
    } else {
        icon_button!("expand_more/24")
    }
}

pub fn window_icon() -> Option<iced::window::Icon> {
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

    cosmic::iced::window::icon::from_rgba(rgba, width, height).ok()
}
