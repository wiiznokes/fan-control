use cosmic::{
    iced::{self, keyboard, touch},
    iced_core::{event::Status, widget::OperationOutputWrapper, Size},
    iced_widget,
};
use iced_widget::core::{
    self, event,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    overlay, renderer,
    widget::{Operation, Tree},
    Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Widget,
};

pub struct DropDown<'a, Overlay, Message, Renderer = iced::Renderer>
where
    Overlay: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: core::Renderer,
{
    underlay: Element<'a, Message, Renderer>,
    lazy_overlay: Overlay,
    on_dismiss: Option<Message>,
    show: bool,
}

impl<'a, Overlay, Message, Renderer> DropDown<'a, Overlay, Message, Renderer>
where
    Overlay: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: core::Renderer,
{
    pub fn new<U>(underlay: U, overlay: Overlay) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
    {
        DropDown {
            underlay: underlay.into(),
            lazy_overlay: overlay,
            show: false,
            on_dismiss: None,
        }
    }
}

impl<'a, Overlay, Message, Renderer> DropDown<'a, Overlay, Message, Renderer>
where
    Overlay: Fn() -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: core::Renderer,
{
    #[must_use]
    pub fn show(mut self, show: bool) -> Self {
        self.show = show;
        self
    }

    #[must_use]
    pub fn on_dismiss(mut self, message: Option<Message>) -> Self {
        self.on_dismiss = message;
        self
    }
}

impl<'a, Content, Message, Renderer> Widget<Message, Renderer>
    for DropDown<'a, Content, Message, Renderer>
where
    Content: 'a + Fn() -> Element<'a, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + core::Renderer,
{
    fn width(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> Length {
        self.underlay.as_widget().height()
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        self.underlay.as_widget().layout(renderer, limits)
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

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&(self.lazy_overlay)())]
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut [&mut self.underlay, &mut (self.lazy_overlay)()]);
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

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        if !self.show {
            return self
                .underlay
                .as_widget_mut()
                .overlay(&mut state.children[0], layout, renderer);
        }

        if state.children.len() == 2 {
            let bounds = layout.bounds();

            Some(overlay::Element::new(
                bounds.position(),
                Box::new(DropDownOverlay::new(
                    &mut state.children[1],
                    (self.lazy_overlay)(),
                    &self.on_dismiss,
                    bounds,
                )),
            ))
        } else {
            None
        }
    }
}

impl<'a, Content, Message, Renderer> From<DropDown<'a, Content, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Content: 'a + Fn() -> Element<'a, Message, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + core::Renderer,
{
    fn from(drop_down: DropDown<'a, Content, Message, Renderer>) -> Self {
        Element::new(drop_down)
    }
}

struct DropDownOverlay<'a, 'b, Message, Renderer = iced::Renderer>
where
    Message: Clone,
{
    state: &'b mut Tree,
    element: Element<'a, Message, Renderer>,
    on_dismiss: &'b Option<Message>,
    underlay_bounds: Rectangle,
}

impl<'a, 'b, Message, Renderer> DropDownOverlay<'a, 'b, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer,
{
    fn new(
        state: &'b mut Tree,
        element: Element<'a, Message, Renderer>,
        on_dismiss: &'b Option<Message>,
        underlay_bounds: Rectangle,
    ) -> Self {
        DropDownOverlay {
            state,
            element,
            on_dismiss,
            underlay_bounds,
        }
    }
}

impl<'a, 'b, Message, Renderer> core::Overlay<Message, Renderer>
    for DropDownOverlay<'a, 'b, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer,
{
    fn layout(&self, renderer: &Renderer, bounds: Size, position: Point) -> Node {
        let limits = Limits::new(Size::ZERO, bounds);
        let max_size = limits.max();

        let mut content = self.element.as_widget().layout(renderer, &limits);

        // Try to stay inside borders
        let mut position = position;
        if position.x + content.size().width > bounds.width {
            position.x = f32::max(0.0, position.x - content.size().width);
        }
        if position.y + content.size().height > bounds.height {
            position.y = f32::max(0.0, position.y - content.size().height);
        }

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
        self.element
            .as_widget()
            .draw(self.state, renderer, theme, style, layout, cursor, &bounds);
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
}
