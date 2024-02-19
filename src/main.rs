pub mod examples;
// use components::iterator::Iterator;
// use components::forms::FormsAndInput;
// use components::controlflow::ControlFlow;
// use examples::error_handling::NumericInput;
// use examples::parent_child_communication::ButtonC;
use examples::passing_children::*;
use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! { <WrapsChildren>"A" "B" "C"</WrapsChildren> }
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
