use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| {
        set_value(
            event_target_value(&ev)
                .parse::<i32>(),
        )
    };

    view! {
        <label>
            "Type a number (or not!)" <input type="number" on:input=on_input/>
            <p>"You entered " <strong>{value}</strong></p>
        </label>
    }
}
