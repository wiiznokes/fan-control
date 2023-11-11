//! A modal for showing elements as an overlay on top of another.
//!
//! *This API requires the following crate features to be activated: ``context_menu``*

use super::widget::State;
use iced::{
    self,
    advanced::{
        layout::{Limits, Node},
        widget::Tree,
        Clipboard, Layout, Overlay, Shell,
    },
    event::Status,
    keyboard,
    mouse::{self, Cursor},
    overlay, touch, Color, Element, Event, Point, Rectangle, Size,
};

use iced::advanced::renderer;

use super::style::StyleSheet;

/// The overlay of the [`DropDownMenu`](crate::native::DropDownMenu).
#[allow(missing_debug_implementations)]
pub struct DropDownMenuOverlay<'a, Message, Renderer = iced::Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// The state of the [`DropDownMenuOverlay`](DropDownMenuOverlay).
    tree: &'a mut Tree,
    /// The content of the [`DropDownMenuOverlay`](DropDownMenuOverlay).
    content: Element<'a, Message, Renderer>,
    /// The style of the [`DropDownMenuOverlay`](DropDownMenuOverlay).
    style: <Renderer::Theme as StyleSheet>::Style,
    /// The state shared between [`DropDownMenu`](crate::native::DropDownMenu) and [`DropDownMenuOverlay`](DropDownMenuOverlay).
    state: &'a mut State,
}

impl<'a, Message, Renderer> DropDownMenuOverlay<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`DropDownMenuOverlay`](DropDownMenuOverlay).
    pub(crate) fn new<C>(
        tree: &'a mut Tree,
        content: C,
        style: <Renderer::Theme as StyleSheet>::Style,
        state: &'a mut State,
    ) -> Self
    where
        C: Into<Element<'a, Message, Renderer>>,
    {
        DropDownMenuOverlay {
            tree,
            content: content.into(),
            style,
            state,
        }
    }

    /// Turn this [`DropDownMenuOverlay`] into an overlay
    /// [`Element`](overlay::Element).
    pub fn overlay(self, position: Point) -> overlay::Element<'a, Message, Renderer> {
        overlay::Element::new(position, Box::new(self))
    }
}

impl<'a, Message, Renderer> Overlay<Message, Renderer> for DropDownMenuOverlay<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn layout(&self, renderer: &Renderer, bounds: Size, position: Point) -> Node {
        let limits = Limits::new(Size::ZERO, bounds);
        let max_size = limits.max();

        let mut content = self.content.as_widget().layout(renderer, &limits);
        content.move_to(position);

        Node::with_children(max_size, vec![content])
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
    ) {
        let bounds = layout.bounds();

        let style_sheet = theme.active(self.style);

        // Background
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: (0.0).into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            style_sheet.background,
        );

        let content_layout = layout
            .children()
            .next()
            .expect("Native: Layout should have a content layout.");

        // Modal
        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            &bounds,
        );
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) -> Status {
        let layout_children = layout
            .children()
            .next()
            .expect("Native: Layout should have a content layout.");

        let status = match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                if key_code == keyboard::KeyCode::Escape {
                    self.state.show = false;
                    Status::Captured
                } else {
                    Status::Ignored
                }
            }

            Event::Mouse(mouse::Event::ButtonPressed(
                mouse::Button::Left | mouse::Button::Right,
            ))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(layout_children.bounds()) {
                    Status::Ignored
                } else {
                    self.state.show = false;
                    Status::Captured
                }
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                // close when released because because button send message on release
                self.state.show = false;
                if cursor.is_over(layout_children.bounds()) {
                    Status::Ignored
                } else {
                    Status::Captured
                }
            }

            _ => Status::Ignored,
        };

        match status {
            Status::Ignored => self.content.as_widget_mut().on_event(
                self.tree,
                event,
                layout_children,
                cursor,
                renderer,
                clipboard,
                shell,
                &layout.bounds(),
            ),
            Status::Captured => Status::Captured,
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout
                .children()
                .next()
                .expect("Native: Layout should have a content layout."),
            cursor,
            viewport,
            renderer,
        )
    }
}
