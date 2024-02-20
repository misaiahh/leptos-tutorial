use leptos::*;

#[component]
pub fn app() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let html =
        "<p>This HTML will be injected.</p>";

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }

            // class:red=move || count() % 2 == 1
            // or with tuple syntax
            class=("red", move || count() % 2 == 1)
            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", count() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", count)
        >
            "Click me: "
            {count}
        </button>
        <progress
            max="50"
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            value=count
        ></progress>
        <div inner_html=html></div>
    }
}
