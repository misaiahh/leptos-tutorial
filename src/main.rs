use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! { <div></div> }
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
