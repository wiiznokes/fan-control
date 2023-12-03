use std::path::PathBuf;

use cosmic::{
    widget::{self, icon::Handle, Icon},
    Element,
};
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
    cosmic::widget::button::icon(get_handle_icon(name))
        .on_press(message)
        .into()
}

pub fn my_icon(name: &str) -> Icon {
    widget::icon::icon(get_handle_icon(name))
}

fn get_handle_icon(name: &str) -> Handle {
    unsafe {
        BUF.clear();
        BUF.insert_str(0, RESSOURCE_PATH);
        BUF.insert_str(BUF.len(), name);
        BUF.insert_str(BUF.len(), EXTENSION);
    };

    let path = format!("{}{}{}", RESSOURCE_PATH, name, EXTENSION);

    cosmic::widget::icon::from_path(PathBuf::from(path))
}
