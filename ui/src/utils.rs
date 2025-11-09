#[cfg(target_os = "windows")]
pub fn setup_wgpu() {
    // unsafe {
    //     std::env::set_var("ICED_BACKEND", "tiny-skia");
    // }
    // debug!("WGPU: Forcing OpenGL backend for graphics.");
}

#[cfg(not(target_os = "windows"))]
pub fn setup_wgpu() {
    debug!("WGPU: Using default graphics backend.");
}

pub trait ApplyMaybe {
    /// Apply a function which takes the parameter by value.
    fn apply_maybe<F: FnOnce(Self) -> Self>(self, condition: bool, f: F) -> Self
    where
        Self: Sized,
    {
        if condition { f(self) } else { self }
    }
}

impl<T: ?Sized> ApplyMaybe for T {
    // use default definitions...
}
