pub trait RemoveElem<T> {
    fn remove_elem<F>(&mut self, predicate: F) -> Option<T>
    where
        F: Fn(&T) -> bool;
}

impl<T> RemoveElem<T> for Vec<T> {
    fn remove_elem<F>(&mut self, predicate: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        self.iter()
            .position(predicate)
            .map(|index| self.remove(index))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MyOption<T>
where
    T: ToString,
{
    Some(T),
    None,
}

impl<T> ToString for MyOption<T>
where
    T: ToString,
{
    fn to_string(&self) -> String {
        match self {
            MyOption::Some(t) => t.to_string(),
            MyOption::None => fl!("none"),
        }
    }
}

impl<T> From<MyOption<T>> for Option<T>
where
    T: ToString,
{
    fn from(value: MyOption<T>) -> Self {
        match value {
            MyOption::Some(value) => Some(value),
            MyOption::None => None,
        }
    }
}
pub mod input {

    use crate::{id::Id, node::Input};

    use super::MyOption;

    impl From<String> for MyOption<Input> {
        fn from(value: String) -> Self {
            MyOption::Some(Input {
                id: Id::default(),
                name: value,
            })
        }
    }

    impl From<Option<String>> for MyOption<Input> {
        fn from(value: Option<String>) -> Self {
            match value {
                Some(value) => MyOption::Some(Input {
                    id: Id::default(),
                    name: value,
                }),
                None => MyOption::None,
            }
        }
    }
}

pub mod hardware {

    use std::rc::Rc;

    use hardware::HardwareInfoTrait;

    use super::MyOption;

    /*
    impl From<&Rc<TempH>> for Input<> {
        fn from(value: &Rc<TempH>) -> Self {
            Input {
                id: value.name.clone(),
                name: value.hardware_id.clone(),
            }
        }
    }
    impl From<&Rc<ControlH>> for Input<String> {
        fn from(value: &Rc<ControlH>) -> Self {
            Input {
                id: value.name.clone(),
                name: value.hardware_id.clone(),
            }
        }
    }
    impl From<&Rc<FanH>> for Input<String> {
        fn from(value: &Rc<FanH>) -> Self {
            Input {
                id: value.name.clone(),
                name: value.hardware_id.clone(),
            }
        }
    }
     */

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct HardwareInfo {
        pub name: String,
        pub id: String,
        pub info: String,
    }

    impl From<Option<String>> for MyOption<HardwareInfo> {
        fn from(value: Option<String>) -> Self {
            match value {
                Some(value) => Self::Some(HardwareInfo {
                    name: value,
                    id: Default::default(),
                    info: Default::default(),
                }),
                None => Self::None,
            }
        }
    }

    impl<T: HardwareInfoTrait> From<&Rc<T>> for HardwareInfo {
        fn from(value: &Rc<T>) -> Self {
            HardwareInfo {
                name: value.name().clone(),
                id: value.id().clone(),
                info: value.info().clone(),
            }
        }
    }

    impl ToString for HardwareInfo {
        fn to_string(&self) -> String {
            self.name.clone()
        }
    }

    pub fn availlable_hardware<'a, H: 'a>(
        hardware_id: &'a Option<String>,
        hardwares: &'a [Rc<H>],
        one_ref: bool,
    ) -> Vec<MyOption<HardwareInfo>>
    where
        H: HardwareInfoTrait,
    {
        let mut hardware_options: Vec<_> = hardwares
            .iter()
            .filter_map(|h| {
                if one_ref {
                    // we leverage rc to know if this specific hardware
                    // is already in use by one node
                    if Rc::strong_count(h) > 1 {
                        return None;
                    }
                }

                match hardware_id {
                    Some(node_hardware_id) => {
                        if node_hardware_id == h.id() {
                            None
                        } else {
                            Some(MyOption::Some(h.into()))
                        }
                    }
                    None => Some(MyOption::Some(h.into())),
                }
            })
            .collect();

        if hardware_id.is_some() {
            hardware_options.insert(0, MyOption::None);
        }

        hardware_options
    }
}
