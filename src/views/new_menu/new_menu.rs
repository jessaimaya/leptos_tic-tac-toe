use leptos::html::{div, h3, button, img};
use leptos::IntoView;
use crate::components::headings::headings::{heading_component, HeadingSize};

stylance::import_crate_style!(styles, "src/views/new_menu/new_menu.module.css");
pub fn new_menu() -> impl IntoView {
    div()
        .attr("class", styles::menu)
        .child(
            (
                tokens(),
                pick_player(),
                option_buttons(),
            )
        )
}

fn tokens() -> impl IntoView {
    div().attr("class", styles::row_tokens)
        .child((
            img().attr("src", "assets/images/cross_full.svg"),
            img().attr("src", "assets/images/circle_full.svg"),
        ))
}

fn pick_player() -> impl IntoView {
    div().attr("class", styles::row_pick)
        .child((
           h3().attr("class",
                     format!("{} {}",
                     heading_component(HeadingSize::S),styles::title))
               .child("PICK PLAYER 1’S MARK"),
            players(),
            h3().attr("class", styles::subtitle)
                .child("REMEMBER : X GOES FIRST")
        ))
}

fn players() -> impl IntoView {
    div().attr("class", styles::players_wrapper)
        .child((
                button().attr("class", styles::cross_player)
            ))
}

fn option_buttons() -> impl IntoView {
    div().attr("class", styles::row_options)
        .child((
            button().attr("class", styles::btn_cpu),
            button().attr("class", styles::btn_player),
        ))
}
