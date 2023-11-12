use std::rc::Rc;

use data::{
    id::Id,
    node::{Node, Nodes},
};
use hardware::{ControlH, FanH, TempH};
use iced::{widget::PickList, Element, Length};

use crate::AppMsg;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdName<I> {
    pub id: I,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pick<I> {
    Some(IdName<I>),
    None,
}

impl<I> Pick<I> {
    pub fn new(name: &str, id: &I) -> Self
    where
        I: Clone,
    {
        Pick::Some(IdName {
            name: name.to_string(),
            id: id.clone(),
        })
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Pick::None => None,
            Pick::Some(IdName { name, .. }) => Some(name.clone()),
        }
    }

    pub fn id(&self) -> Option<I>
    where
        I: Clone,
    {
        match self {
            Pick::None => None,
            Pick::Some(IdName { id, .. }) => Some(id.clone()),
        }
    }

    pub fn display_only(optionnal_name: &Option<String>) -> Option<Self>
    where
        I: Default,
    {
        let pick = match optionnal_name {
            Some(name) => Self::Some(IdName {
                name: name.clone(),
                id: I::default(),
            }),
            None => Self::None,
        };
        Some(pick)
    }

    pub fn to_couple(&self) -> Option<(I, String)>
    where
        I: Clone,
    {
        match self {
            Pick::Some(IdName { id, name }) => Some((id.clone(), name.clone())),
            Pick::None => None,
        }
    }
}

impl<I> ToString for Pick<I> {
    fn to_string(&self) -> String {
        match &self {
            Pick::Some(IdName { name, .. }) => name.clone(),
            Pick::None => "None".into(),
        }
    }
}

pub fn pick_input<'a>(
    node: &'a Node,
    nodes: &'a Nodes,
    current_input: &Option<String>,
    add_none: bool,
    // todo: try to remove this box with sized
    map_pick: Box<dyn Fn(Id, Pick<Id>) -> AppMsg>,
) -> Element<'a, AppMsg> {
    let mut input_options = nodes
        .values()
        .filter(|n| {
            node.node_type
                .to_light()
                .allowed_dep()
                .contains(&n.node_type.to_light())
                && !node
                    .inputs
                    .iter()
                    .map(|i| i.0)
                    .collect::<Vec<_>>()
                    .contains(&n.id)
        })
        .map(|n| Pick::new(n.name(), &n.id))
        .collect::<Vec<_>>();

    if add_none && current_input.is_some() {
        input_options.insert(0, Pick::None);
    }

    PickList::new(
        input_options,
        Pick::display_only(current_input),
        move |pick| map_pick(node.id, pick),
    )
    .width(Length::Fill)
    .into()
}

pub fn pick_hardware<'a, P: 'a>(
    node: &'a Node,
    hardwares: &'a [Rc<P>],
    one_ref: bool,
) -> Element<'a, AppMsg>
where
    Pick<String>: From<&'a Rc<P>>,
{
    let hardware_id = node.hardware_id().unwrap();

    let mut hardware_options = hardwares
        .iter()
        .filter_map(|h| {
            if one_ref {
                // we leverage rc to know if this specific hardware
                // is already in use by one node
                if Rc::strong_count(h) > 1 {
                    return None;
                }
            }

            let pick: Pick<String> = h.into();

            match (hardware_id, &pick.name()) {
                (None, None) => Some(pick),
                (None, Some(_)) => Some(pick),
                (Some(_), None) => Some(pick),
                (Some(name1), Some(name2)) => {
                    if name1 == name2 {
                        None
                    } else {
                        Some(pick)
                    }
                }
            }
        })
        .collect::<Vec<Pick<String>>>();

    if hardware_id.is_some() {
        hardware_options.insert(0, Pick::None);
    }

    PickList::new(hardware_options, Pick::display_only(hardware_id), |pick| {
        AppMsg::ChangeHardware(node.id, pick)
    })
    .width(Length::Fill)
    .into()
}

impl From<&Rc<TempH>> for Pick<String> {
    fn from(value: &Rc<TempH>) -> Self {
        Pick::new(&value.name, &value.hardware_id)
    }
}
impl From<&Rc<ControlH>> for Pick<String> {
    fn from(value: &Rc<ControlH>) -> Self {
        Pick::new(&value.name, &value.hardware_id)
    }
}
impl From<&Rc<FanH>> for Pick<String> {
    fn from(value: &Rc<FanH>) -> Self {
        Pick::new(&value.name, &value.hardware_id)
    }
}
