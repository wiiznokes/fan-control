use std::rc::Rc;

use crate::{
    fl,
    message::{ModifNodeMsg},
};
use cosmic::{iced_core::Length, iced_widget::PickList, Element};
use data::{
    node::{Input, Node},
};
use hardware::{ControlH, FanH, TempH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pick<I> {
    Some(Input<I>),
    None,
}

impl<I> Pick<I> {
    pub fn new(name: &str, id: &I) -> Self
    where
        I: Clone,
    {
        Pick::Some(Input {
            name: name.to_string(),
            id: id.clone(),
        })
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Pick::None => None,
            Pick::Some(Input { name, .. }) => Some(name.clone()),
        }
    }

    pub fn id(&self) -> Option<I>
    where
        I: Clone,
    {
        match self {
            Pick::None => None,
            Pick::Some(Input { id, .. }) => Some(id.clone()),
        }
    }

    /// generate a default id
    pub fn display_only(optionnal_name: &Option<String>) -> Option<Self>
    where
        I: Default,
    {
        let pick = match optionnal_name {
            Some(name) => Self::Some(Input {
                name: name.clone(),
                id: I::default(),
            }),
            None => Self::None,
        };
        Some(pick)
    }
}

impl<I> ToString for Pick<I> {
    fn to_string(&self) -> String {
        match &self {
            Pick::Some(Input { name, .. }) => name.clone(),
            Pick::None => fl!("none"),
        }
    }
}

pub fn pick_hardware<'a, P: 'a>(
    node: &'a Node,
    hardwares: &'a [Rc<P>],
    one_ref: bool,
) -> Element<'a, ModifNodeMsg>
where
    Pick<String>: From<&'a Rc<P>>,
{
    let hardware_id = node.hardware_id();

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
        ModifNodeMsg::ChangeHardware(pick)
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
