use leptos::html::div;
use leptos::IntoView;
use crate::views::new_menu::new_menu::new_menu;

stylance::import_crate_style!(styles, "src/components/layout/layout.module.css");
pub fn layout_component() -> impl IntoView {

    div().attr("class", styles::layout).child(
        new_menu()
    )
}
