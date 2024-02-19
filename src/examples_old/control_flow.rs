use leptos::*;

#[component]
pub fn ControlFlow() -> impl IntoView {
    let (value, _set_value) = create_signal(3);
    let is_odd = move || value() & 1 == 1;

    view! {
        <main>
            {move || match is_odd() {
                true if value() == 1 => view! { <pre>"One"</pre> }.into_any(),
                false if value() == 2 => view! { <p>"Two"</p> }.into_any(),
                _ => view! { <textarea>{value()}</textarea> }.into_any(),
            }}

        </main>
    }
}
