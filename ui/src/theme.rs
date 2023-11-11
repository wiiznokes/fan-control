use iced::{
    color,
    widget::text_input,
    widget::{container, scrollable},
    Color,
};

/// apply custom theme
///
/// .style(iced::theme::Container::Custom(Box::new(CustomContainerStyle::Item)))

pub enum CustomContainerStyle {
    Item,
    Background,
}

impl container::StyleSheet for CustomContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        match self {
            CustomContainerStyle::Item => container::Appearance {
                background: Some(color!(0xffffff).into()),
                border_radius: 0f32.into(),
                border_width: 2f32,
                border_color: color!(0xf0ff00),
                ..Default::default()
            },
            CustomContainerStyle::Background => container::Appearance {
                background: Some(color!(0x000000).into()),
                border_radius: 0f32.into(),
                border_width: 2f32,
                border_color: color!(0xf0ff00),
                ..Default::default()
            },
        }
    }
}

pub enum CustomScrollableStyle {
    Background,
}

impl scrollable::StyleSheet for CustomScrollableStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        let palette = style.extended_palette();

        scrollable::Scrollbar {
            background: Some(color!(0xffffff).into()),
            border_radius: 2.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: palette.background.strong.color,
                border_radius: 2.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> scrollable::Scrollbar {
        if is_mouse_over_scrollbar {
            let palette = style.extended_palette();

            scrollable::Scrollbar {
                background: Some(color!(0xffffff).into()),
                border_radius: 2.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: scrollable::Scroller {
                    color: palette.primary.strong.color,
                    border_radius: 2.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            }
        } else {
            self.active(style)
        }
    }
}

pub enum CustomTextInputStyle {
    Error,
}

impl text_input::StyleSheet for CustomTextInputStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        match self {
            CustomTextInputStyle::Error => text_input::Appearance {
                background: palette.background.base.color.into(),
                border_radius: 2.0.into(),
                border_width: 1.0,
                border_color: palette.danger.strong.color,
                icon_color: palette.background.weak.text,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        match self {
            CustomTextInputStyle::Error => text_input::Appearance {
                background: palette.background.base.color.into(),
                border_radius: 2.0.into(),
                border_width: 1.0,
                border_color: palette.danger.strong.color,
                icon_color: palette.background.weak.text,
            },
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        match self {
            CustomTextInputStyle::Error => text_input::Appearance {
                background: palette.background.base.color.into(),
                border_radius: 2.0.into(),
                border_width: 1.0,
                border_color: palette.danger.strong.color,
                icon_color: palette.background.weak.text,
            },
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        let palette = style.extended_palette();

        match self {
            CustomTextInputStyle::Error => palette.background.strong.color,
        }
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        let palette = style.extended_palette();

        match self {
            CustomTextInputStyle::Error => palette.background.base.text,
        }
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        let palette = style.extended_palette();

        match self {
            CustomTextInputStyle::Error => palette.primary.weak.color,
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        match self {
            CustomTextInputStyle::Error => text_input::Appearance {
                background: palette.background.weak.color.into(),
                border_radius: 2.0.into(),
                border_width: 1.0,
                border_color: palette.background.strong.color,
                icon_color: palette.background.strong.color,
            },
        }
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        self.placeholder_color(style)
    }
}
