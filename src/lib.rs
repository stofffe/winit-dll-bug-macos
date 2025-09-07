#[unsafe(no_mangle)]
pub fn access_window_functions(window: &winit::window::Window) {
    println!(
        "The lib is working: Window size is {:?}",
        window.inner_size()
    );
}
