use leptos::*;
mod part_one;
// use part_one::c_components_and_props::app;
// use crate::part_one::c_components_and_props::App;
use crate::part_one::e_iterating_over_more_complex_data::App;

fn main() {
    _ = console_log::init_with_level(
        log::Level::Debug,
    );
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
