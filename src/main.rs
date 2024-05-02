use components::app::app::app_component;
use leptos::*;
mod components;
mod views;

fn main() {
    mount_to_body(app_component)
}
