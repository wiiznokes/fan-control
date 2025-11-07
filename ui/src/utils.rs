#[cfg(target_os = "windows")]
pub fn setup_wgpu() {
    unsafe {
        std::env::set_var("ICED_BACKEND", "tiny-skia");
    }
    debug!("WGPU: Forcing OpenGL backend for graphics.");
}

#[cfg(not(target_os = "windows"))]
pub fn setup_wgpu() {
    debug!("WGPU: Using default graphics backend.");
}
