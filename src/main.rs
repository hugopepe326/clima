mod api;
mod app;
mod components;
mod format;
mod geolocate;
mod js;
mod models;
mod state;
mod storage;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
