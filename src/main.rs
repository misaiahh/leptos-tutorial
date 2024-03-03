use leptos::*;
mod part_1;

// use crate::part_1::chapter_3::control_flow::App;
// use crate::part_1::chapter_3::error_handling::NumericInput;
// use crate::part_1::chapter_3::parent_child_communication::App;
// use crate::part_1::chapter_3::passing_children_to_components::TakesChildren;
use crate::part_1::chapter_3::passing_children_to_components::WrapsChildren;

fn main() {
  _ = console_log::init_with_level(
    log::Level::Debug,
  );
  console_error_panic_hook::set_once();

  //   mount_to_body(|| view! { <App/> });

  //   mount_to_body(|| {
  //     view! {
  //         <TakesChildren render_prop=|| {
  //             view! { <p>"Hi, there!"</p> }
  //         }>
  //             // these get passed to `children`
  //             "Some text" <span>"A span"</span>
  //         </TakesChildren>
  //     }
  //   });

  mount_to_body(|| {
    view! { <WrapsChildren>"A" "B" "C"</WrapsChildren> }
  });
}
