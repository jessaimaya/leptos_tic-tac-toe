use leptos::html::section;
use leptos::IntoView;

use crate::components::layout::layout::layout_component;

stylance::import_crate_style!(styles, "src/components/app/app.module.css");
pub fn app_component() -> impl IntoView {
        section()
        .attr("class", styles::app)
        .child(layout_component())
}
