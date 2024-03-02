use leptos::*;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    let (data, _set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: create_rw_signal(10),
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: create_rw_signal(20),
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: create_rw_signal(15),
        },
    ]);
    view! {
        <button on:click=move |_| {
            data.with(|data| {
                for row in data {
                    row.value.update(|value| *value *= 2);
                }
            });
            logging::log!("{:?}", data.get());
        }>"Update Values"</button>

        <For each=data key=|state| state.key.clone() let:child>
            <p>{child.value}</p>
        </For>
    }
}
