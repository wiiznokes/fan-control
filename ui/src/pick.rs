use std::rc::Rc;

use data::{
    id::Id,
    node::{Node, Nodes},
};
use hardware::{ControlH, FanH, TempH};
use iced::{widget::PickList, Element, Length};

use crate::AppMsg;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pick<I> {
    pub name: Option<String>,
    pub id: Option<I>,
}

impl<I> Pick<I> {
    pub fn none() -> Self {
        Pick {
            name: None,
            id: None,
        }
    }
    pub fn with_name(name: &str) -> Self {
        Pick {
            name: Some(name.to_string()),
            id: None,
        }
    }
    pub fn with_name_id(name: &str, id: &I) -> Self
    where
        I: Clone,
    {
        Pick {
            name: Some(name.to_string()),
            id: Some(id.clone()),
        }
    }
    pub fn selected(option: &Option<String>) -> Option<Self> {
        match option {
            Some(str) => Some(Self::with_name(str)),
            None => Some(Self::none()),
        }
    }
}

impl<I> ToString for Pick<I> {
    fn to_string(&self) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => "None".into(),
        }
    }
}

pub fn pick_input<'a>(
    node: &'a Node,
    nodes: &'a Nodes,
    current_input: &Option<String>,
) -> Element<'a, AppMsg> {
    let mut input_options = nodes
        .values()
        .filter(|n| {
            node.node_type
                .to_light()
                .allowed_dep()
                .contains(&n.node_type.to_light())
                && !node.inputs.contains(&n.id)
        })
        .map(|n| Pick::with_name_id(n.name(), &n.id))
        .collect::<Vec<_>>();

    if current_input.is_some() {
        input_options.insert(0, Pick::none());
    }

    PickList::new(input_options, Pick::selected(current_input), |pick| {
        AppMsg::InputReplaced(node.id, pick)
    })
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

            match (hardware_id, &pick.name) {
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
        hardware_options.insert(0, Pick::none());
    }

    PickList::new(hardware_options, Pick::selected(hardware_id), |pick| {
        AppMsg::HardwareIdChange(node.id, pick.name)
    })
    .width(Length::Fill)
    .into()
}

impl From<&Rc<TempH>> for Pick<String> {
    fn from(value: &Rc<TempH>) -> Self {
        Pick::with_name_id(&value.name, &value.hardware_id)
    }
}
impl From<&Rc<ControlH>> for Pick<String> {
    fn from(value: &Rc<ControlH>) -> Self {
        Pick::with_name_id(&value.name, &value.hardware_id)
    }
}
impl From<&Rc<FanH>> for Pick<String> {
    fn from(value: &Rc<FanH>) -> Self {
        Pick::with_name_id(&value.name, &value.hardware_id)
    }
}
