use leptos::*;

#[component]
fn ProgressBar(
    progress: ReadSignal<i32>,
) -> impl IntoView {
    view! { <progress max="50" value=progress></progress> }
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button on:click=move |_| {
            set_count.update(|n| *n += 1);
        }>"Click me"</button>
        <ProgressBar progress=count/>
    }
}
