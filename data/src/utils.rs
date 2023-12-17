use crate::node::Input;

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

impl<I> From<String> for MyOption<Input<I>>
where
    I: Default,
{
    fn from(value: String) -> Self {
        MyOption::Some(Input {
            id: I::default(),
            name: value,
        })
    }
}

impl<I> From<Option<String>> for MyOption<Input<I>>
where
    I: Default,
{
    fn from(value: Option<String>) -> Self {
        match value {
            Some(value) => value.into(),
            None => MyOption::None,
        }
    }
}
