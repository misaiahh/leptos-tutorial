use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn ButtonA(
    setter: WriteSignal<bool>,
) -> impl IntoView {
    view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle"</button> }
}
// // <ButtonA setter=set_toggled/>

#[component]
pub fn ButtonB(
    #[prop(into)] on_click: Callback<MouseEvent>,
) -> impl IntoView {
    view! { <button on:click=on_click>"Toggle"</button> }
}
// // <ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>

#[component]
pub fn ButtonB2<F>(on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { <button on:click=on_click>"Toggle"</button> }
}
// // <ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>

#[component]
pub fn ButtonC() -> impl IntoView {
    view! { <button>"Toggle"</button> }
}
// <ButtonC on:click=move |_| set_toggled.update(|value| *value = !*value)/>

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <header>
            <h1>"My Page"</h1>
        </header>
        <main>
            <Content/>
        </main>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ButtonD/>
        </div>
    }
}

#[component]
pub fn ButtonD() -> impl IntoView {
    let setter =
        use_context::<WriteSignal<bool>>()
            .expect(
            "to have found the setter provided",
        );
    view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle"</button> }
}

// #[component]
// fn App() -> impl IntoView {
//     let (toggled, set_toggled) =
//         create_signal(false);
//     provide_context(set_toggled);
//     view! {
//         <p>"Toggled? " {toggled}</p>
//         <Layout/>
//     }
// }
