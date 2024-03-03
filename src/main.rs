use leptos::*;
mod part_1;

// use crate::part_1::chapter_3::control_flow::App;
// use crate::part_1::chapter_3::error_handling::NumericInput;
use crate::part_1::chapter_3::parent_child_communication::App;

fn main() {
  _ = console_log::init_with_level(
    log::Level::Debug,
  );
  console_error_panic_hook::set_once();
  mount_to_body(|| view! { <App/> });
}
