use std::path::PathBuf;

use cosmic::Element;
use once_cell::sync::Lazy;

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

static RESSOURCE_PATH: &str = "./ressource/icons/";
static EXTENSION: &str = ".svg";

static mut BUF: Lazy<String> = Lazy::new(|| String::with_capacity(50));

pub fn icon_button<M: 'static + Clone>(name: &str, message: M) -> Element<M> {
    unsafe {
        BUF.clear();
        BUF.insert_str(0, RESSOURCE_PATH);
        BUF.insert_str(BUF.len(), name);
        BUF.insert_str(BUF.len(), EXTENSION);
    };

    let path = format!("{}{}{}", RESSOURCE_PATH, name, EXTENSION);

    let handle = cosmic::widget::icon::from_path(PathBuf::from(path));

    cosmic::widget::button::icon(handle)
        .on_press(message)
        .into()
}
