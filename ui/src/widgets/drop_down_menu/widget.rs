use std::{slice::Iter, borrow::Cow};

use iced::{
    self,
    advanced::renderer,
    advanced::widget::{tree, Operation, Tree},
    advanced::{
        layout::{Limits, Node},
        Clipboard, Layout, Shell, Widget,
    },
    event,
    mouse::{self, Button, Cursor},
    overlay, Event, Length, Point, Rectangle, widget::Column,
    Element
};

use super::{overlay::DropDownMenuOverlay, style::StyleSheet};


pub struct DropDownMenu<'a, T, ItemView, Message, Renderer = iced::Renderer>
where
    [T]: ToOwned<Owned = Vec<T>>,
    ItemView: Fn(&T) -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    on_item_click: Option<Box<dyn Fn(T) -> Message + 'a>>,
    on_dismiss: Option<Box<dyn Fn() -> Message + 'a>>,
    items: Cow<'a, [T]>,
    show: bool,
    underlay: Element<'a, Message, Renderer>,
    item_view: ItemView,
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, T: 'a, ItemView, Message, Renderer> DropDownMenu<'a, T, ItemView, Message, Renderer>
where
    [T]: ToOwned<Owned = Vec<T>>,
    ItemView: Fn(&T) -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    pub fn new<U>(underlay: U, item_view: ItemView, items: impl Into<Cow<'a, [T]>>, show: bool) -> Self
    where
        U: Into<Element<'a, Message, Renderer>>
    {
        DropDownMenu {
            show,
            underlay: underlay.into(),
            item_view,
            style: <Renderer::Theme as StyleSheet>::Style::default(),
            on_item_click: None,
            on_dismiss: None,
            items: items.into(),
        }
    }

    /// Sets the style of the [`DropDownMenu`](DropDownMenu).
    #[must_use]
    pub fn style(mut self, style: <Renderer::Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a, T, ItemView, Message, Renderer> Widget<Message, Renderer>
    for DropDownMenu<'a, T, ItemView, Message, Renderer>
where
    [T]: ToOwned<Owned = Vec<T>>,
    ItemView: Fn(&T) -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
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

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {

        /*
        let list = self.items.iter().map(|i| {
            (self.item_view)(i)
        }).collect();
        let col: Element<Message> = Column::with_children(list).into();
 */

        vec![Tree::new(&self.underlay)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay]);
    }

    fn operate<'b>(
        &'b self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
      

        if self.show {
            todo!()
        } else {
            self.underlay
                .as_widget()
                .operate(&mut state.children[0], layout, renderer, operation);
        }
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
        let s: &mut State = state.state.downcast_mut();

        if !s.show {
            return self
                .underlay
                .as_widget_mut()
                .overlay(&mut state.children[0], layout, renderer);
        }

        let position = s.cursor_position;
        let content = (self.overlay)();
        content.as_widget().diff(&mut state.children[1]);

        Some(
            DropDownMenuOverlay::new(&mut state.children[1], content, self.style, s)
                .overlay(position),
        )
    }
}

impl<'a, T, ItemView, Message, Renderer> From<DropDownMenu<'a, T, ItemView, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
[T]: ToOwned<Owned = Vec<T>>,
    ItemView: Fn(&T) -> Element<'a, Message, Renderer>,
    Message: Clone,
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(modal: DropDownMenu<'a, T, ItemView, Message, Renderer>) -> Self {
        Element::new(modal)
    }
}

/// The state of the ``context_menu``.
#[derive(Debug, Default)]
pub(crate) struct State {
}

impl State {
    /// Creates a new [`State`](State) containing the given state data.
    pub const fn new() -> Self {
        Self {
        }
    }
}
