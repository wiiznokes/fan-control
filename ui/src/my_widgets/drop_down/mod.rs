#![allow(clippy::too_many_arguments)]

use cosmic::{
    iced::{self, keyboard, touch},
    iced_core::{widget::OperationOutputWrapper, Size, Vector},
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

use super::{alignment::Alignment, offset::Offset};

pub struct DropDown<'a, Message, Renderer = iced::Renderer>
where
    Message: Clone,
    Renderer: core::Renderer,
{
    underlay: Element<'a, Message, Renderer>,
    overlay: Element<'a, Message, Renderer>,
    on_dismiss: Option<Message>,
    width: Option<Length>,
    height: Length,
    alignment: Alignment,
    offset: Offset,
    expanded: bool,
}

impl<'a, Message, Renderer> DropDown<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer,
{
    pub fn new<U, B>(underlay: U, overlay: B, expanded: bool) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>,
        B: Into<Element<'a, Message, Renderer>>,
    {
        DropDown {
            underlay: underlay.into(),
            overlay: overlay.into(),
            expanded,
            on_dismiss: None,
            width: None,
            height: Length::Shrink,
            alignment: Alignment::Bottom,
            offset: Offset::from(5.0),
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn alignment(mut self, alignment: impl Into<Alignment>) -> Self {
        self.alignment = alignment.into();
        self
    }

    pub fn offset(mut self, offset: impl Into<Offset>) -> Self {
        self.offset = offset.into();
        self
    }
}

impl<'a, Message, Renderer> DropDown<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer,
{
    #[must_use]
    pub fn on_dismiss(mut self, message: Message) -> Self {
        self.on_dismiss = Some(message);
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for DropDown<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + core::Renderer,
{
    fn width(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> Length {
        self.underlay.as_widget().height()
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        self.underlay
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
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
        vec![Tree::new(&self.underlay), Tree::new(&self.overlay)]
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut [&mut self.underlay, &mut self.overlay]);
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
        if !self.expanded {
            return self
                .underlay
                .as_widget_mut()
                .overlay(&mut state.children[0], layout, renderer);
        }

        let bounds = layout.bounds();

        Some(overlay::Element::new(
            bounds.position(),
            Box::new(DropDownOverlay::new(
                &mut state.children[1],
                &mut self.overlay,
                &self.on_dismiss,
                &self.width,
                &self.height,
                &self.alignment,
                &self.offset,
                layout.bounds(),
            )),
        ))
    }
}

impl<'a, Message, Renderer> From<DropDown<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + core::Renderer,
{
    fn from(drop_down: DropDown<'a, Message, Renderer>) -> Self {
        Element::new(drop_down)
    }
}

struct DropDownOverlay<'a, 'b, Message, Renderer = iced::Renderer>
where
    Message: Clone,
{
    state: &'b mut Tree,
    element: &'b mut Element<'a, Message, Renderer>,
    on_dismiss: &'b Option<Message>,
    width: &'b Option<Length>,
    height: &'b Length,
    alignment: &'b Alignment,
    offset: &'b Offset,
    underlay_bounds: Rectangle,
}

impl<'a, 'b, Message, Renderer> DropDownOverlay<'a, 'b, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer,
{
    fn new(
        state: &'b mut Tree,
        element: &'b mut Element<'a, Message, Renderer>,
        on_dismiss: &'b Option<Message>,
        width: &'b Option<Length>,
        height: &'b Length,
        alignment: &'b Alignment,
        offset: &'b Offset,
        underlay_bounds: Rectangle,
    ) -> Self {
        DropDownOverlay {
            state,
            element,
            on_dismiss,
            underlay_bounds,
            width,
            alignment,
            offset,
            height,
        }
    }
}

impl<'a, 'b, Message, Renderer> core::Overlay<Message, Renderer>
    for DropDownOverlay<'a, 'b, Message, Renderer>
where
    Message: Clone,
    Renderer: core::Renderer,
{
    fn layout(
        &mut self,
        renderer: &Renderer,
        bounds: Size,
        position: Point,
        _translation: Vector,
    ) -> Node {
        let limits = Limits::new(Size::ZERO, bounds)
            .width(
                self.width
                    .unwrap_or(Length::Fixed(self.underlay_bounds.width)),
            )
            .height(*self.height);

        let mut node = self
            .element
            .as_widget()
            .layout(self.state, renderer, &limits);

        let position = match self.alignment {
            Alignment::TopStart => Point::new(
                position.x - node.bounds().width - self.offset.x,
                position.y - node.bounds().height + self.underlay_bounds.height - self.offset.y,
            ),
            Alignment::Top => Point::new(
                position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0,
                position.y - node.bounds().height - self.offset.y,
            ),
            Alignment::TopEnd => Point::new(
                position.x + self.underlay_bounds.width + self.offset.x,
                position.y - node.bounds().height + self.underlay_bounds.height - self.offset.y,
            ),
            Alignment::End => Point::new(
                position.x + self.underlay_bounds.width + self.offset.x,
                position.y + self.underlay_bounds.height / 2.0 - node.bounds().height / 2.0,
            ),
            Alignment::BottomEnd => Point::new(
                position.x + self.underlay_bounds.width + self.offset.x,
                position.y + self.offset.y,
            ),
            Alignment::Bottom => Point::new(
                position.x + self.underlay_bounds.width / 2.0 - node.bounds().width / 2.0,
                position.y + self.underlay_bounds.height + self.offset.y,
            ),
            Alignment::BottomStart => Point::new(
                position.x - node.bounds().width - self.offset.x,
                position.y + self.offset.y,
            ),
            Alignment::Start => Point::new(
                position.x - node.bounds().width - self.offset.x,
                position.y + self.underlay_bounds.height / 2.0 - node.bounds().height / 2.0,
            ),
        };

        node.move_to(position);

        node
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
                    if !cursor.is_over(layout.bounds()) && !cursor.is_over(self.underlay_bounds) {
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
