use cosmic::{
    iced::{self, keyboard, touch},
    iced_core::{
        alignment::{Horizontal, Vertical},
        widget::OperationOutputWrapper,
    },
    iced_widget,
};
use iced_widget::core::{
    self, event,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    overlay, renderer,
    widget::{Operation, Tree},
    Clipboard, Element, Event, Layout, Length, Rectangle, Shell, Widget,
};

use iced_widget::core::{layout, Point, Size};

mod anchor;
mod offset;

pub use anchor::Anchor;
pub use offset::Offset;

pub struct FloatingElement<'a, Message, Renderer = iced::Renderer>
where
    Renderer: core::Renderer,
    Message: Clone,
{
    anchor: Anchor,
    offset: Offset,
    hidden: bool,
    underlay: Element<'a, Message, Renderer>,
    element: Element<'a, Message, Renderer>,
    on_dismiss: Option<Message>,
}

impl<'a, Message, Renderer> FloatingElement<'a, Message, Renderer>
where
    Renderer: core::Renderer,
    Message: Clone,
{
    pub fn new<U, B>(underlay: U, element: B, anchor: Anchor) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
        B: Into<Element<'a, Message, Renderer>>,
    {
        FloatingElement {
            anchor,
            offset: 5.0.into(),
            hidden: false,
            underlay: underlay.into(),
            element: element.into(),
            on_dismiss: None,
        }
    }

    /// Hide or unhide the [`Element`] on the [`FloatingElement`].
    #[must_use]
    pub fn hide(mut self, hide: bool) -> Self {
        self.hidden = hide;
        self
    }

    /// Sets the [`Offset`] of the [`FloatingElement`].
    #[must_use]
    pub fn offset<O>(mut self, offset: O) -> Self
    where
        O: Into<Offset>,
    {
        self.offset = offset.into();
        self
    }

    /// Trigger when a click is made outside of the floating element
    #[must_use]
    pub fn on_dismiss(mut self, message: Option<Message>) -> Self {
        self.on_dismiss = message;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for FloatingElement<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: core::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&self.element)]
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut [&mut self.underlay, &mut self.element]);
    }

    fn width(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> Length {
        self.underlay.as_widget().height()
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        self.underlay.as_widget().layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.underlay.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn operate<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<OperationOutputWrapper<Message>>,
    ) {
        self.underlay
            .as_widget()
            .operate(&mut state.children[0], layout, renderer, operation);
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        if self.hidden {
            return self
                .underlay
                .as_widget_mut()
                .overlay(&mut state.children[0], layout, renderer);
        }

        if state.children.len() == 2 {
            let bounds = layout.bounds();

            Some(overlay::Element::new(
                bounds.position(),
                Box::new(FloatingElementOverlay::new(
                    &mut state.children[1],
                    &mut self.element,
                    &self.anchor,
                    &self.offset,
                    &self.on_dismiss,
                    bounds,
                )),
            ))
        } else {
            None
        }
    }
}

impl<'a, Message, Renderer> From<FloatingElement<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + core::Renderer,
{
    fn from(floating_element: FloatingElement<'a, Message, Renderer>) -> Self {
        Element::new(floating_element)
    }
}

struct FloatingElementOverlay<'a, 'b, Message, Renderer: core::Renderer>
where
    Message: Clone,
{
    state: &'b mut Tree,
    element: &'b mut Element<'a, Message, Renderer>,
    anchor: &'b Anchor,
    offset: &'b Offset,
    on_dismiss: &'b Option<Message>,
    underlay_bounds: Rectangle,
}

impl<'a, 'b, Message, Renderer> FloatingElementOverlay<'a, 'b, Message, Renderer>
where
    Renderer: core::Renderer,
    Message: Clone,
{
    /// Creates a new [`FloatingElementOverlay`] containing the given
    /// [`Element`](iced_widget::core::Element).
    fn new(
        state: &'b mut Tree,
        element: &'b mut Element<'a, Message, Renderer>,
        anchor: &'b Anchor,
        offset: &'b Offset,
        on_dismiss: &'b Option<Message>,
        underlay_bounds: Rectangle,
    ) -> Self {
        FloatingElementOverlay {
            state,
            element,
            anchor,
            offset,
            on_dismiss,
            underlay_bounds,
        }
    }
}

impl<'a, 'b, Message, Renderer> core::Overlay<Message, Renderer>
    for FloatingElementOverlay<'a, 'b, Message, Renderer>
where
    Renderer: core::Renderer,
    Message: Clone,
{
    fn layout(&self, renderer: &Renderer, _bounds: Size, position: Point) -> layout::Node {
        // Constrain overlay to fit inside the underlay's bounds
        let limits = layout::Limits::new(Size::ZERO, self.underlay_bounds.size())
            .width(Length::Fill)
            .height(Length::Fill);
        let mut node = self.element.as_widget().layout(renderer, &limits);

        let position = match (self.anchor.vertical, self.anchor.horizontal) {
            (Vertical::Top, Horizontal::Left) => {
                Point::new(position.x + self.offset.x, position.y + self.offset.y)
            }
            (Vertical::Top, Horizontal::Center) => Point::new(
                position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0
                    + self.offset.x,
                position.y + self.offset.y,
            ),
            (Vertical::Top, Horizontal::Right) => Point::new(
                position.x + self.underlay_bounds.width - node.bounds().width - self.offset.x,
                position.y + self.offset.y,
            ),
            (Vertical::Center, Horizontal::Left) => Point::new(
                position.x + self.offset.x,
                position.y + self.underlay_bounds.height / 2.0 - node.bounds().height / 2.0
                    + self.offset.y,
            ),
            (Vertical::Center, Horizontal::Center) => Point::new(
                position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0,
                position.y + self.underlay_bounds.height / 2.0 - node.bounds().height / 2.0,
            ),
            (Vertical::Center, Horizontal::Right) => Point::new(
                position.x + self.underlay_bounds.width - node.bounds().width - self.offset.x,
                position.y + self.underlay_bounds.height / 2.0 - node.bounds().height / 2.0
                    + self.offset.y,
            ),
            (Vertical::Bottom, Horizontal::Left) => Point::new(
                position.x + self.offset.x,
                position.y + self.underlay_bounds.height - node.bounds().height - self.offset.y,
            ),
            (Vertical::Bottom, Horizontal::Center) => Point::new(
                position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0
                    + self.offset.x,
                position.y + self.underlay_bounds.height - node.bounds().height - self.offset.y,
            ),
            (Vertical::Bottom, Horizontal::Right) => Point::new(
                position.x + self.underlay_bounds.width - node.bounds().width - self.offset.x,
                position.y + self.underlay_bounds.height - node.bounds().height - self.offset.y,
            ),
        };

        node.move_to(position);
        node
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<Message>,
    ) -> event::Status {
        if let Some(message) = self.on_dismiss {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                    if key_code == keyboard::KeyCode::Escape {
                        shell.publish(message.clone());
                    }
                }

                Event::Mouse(mouse::Event::ButtonPressed(
                    mouse::Button::Left | mouse::Button::Right,
                ))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if !cursor.is_over(layout.bounds()) {
                        shell.publish(message.clone());
                    }
                }

                _ => {}
            }
        }

        self.element.as_widget_mut().on_event(
            self.state,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        )
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.element
            .as_widget()
            .mouse_interaction(self.state, layout, cursor, viewport, renderer)
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
        self.element
            .as_widget()
            .draw(self.state, renderer, theme, style, layout, cursor, &bounds);
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Renderer>> {
        self.element
            .as_widget_mut()
            .overlay(self.state, layout, renderer)
    }
}
