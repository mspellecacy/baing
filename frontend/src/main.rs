mod api;
mod app;
mod components;
mod pages;
mod router;
mod store;
mod ui_helpers;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
