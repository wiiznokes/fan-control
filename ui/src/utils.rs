use std::path::PathBuf;

use cosmic::widget::{self, icon::Handle, Icon};
use once_cell::sync::Lazy;

static RESSOURCE_PATH: &str = "./ressource/icons/";
static EXTENSION: &str = ".svg";

static mut BUF: Lazy<String> = Lazy::new(|| String::with_capacity(50));

pub fn icon_button<M>(name: &str) -> widget::button::IconButton<M> {
    cosmic::widget::button::icon(get_handle_icon(name))
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
