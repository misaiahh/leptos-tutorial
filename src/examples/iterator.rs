use leptos::*;
use std::vec;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn Iterator() -> impl IntoView {
    // let (count, set_count) = create_signal(0);
    // let double_count = move || count() * 2;
    // let values = vec![0, 1, 2, 3, 4, 5];
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 1,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 2,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 3,
        },
    ]);

    view! {
        <button on:click=move |_| {
            set_data
                .update(|data| {
                    for row in data {
                        row.value *= 2;
                    }
                });
            logging::log!("{:?}", data());
        }>"Click me"</button>
        <For
            each=move || data().into_iter().enumerate()
            key=|(_, database_entry)| database_entry.key.clone()
            children=move |(index, _)| {
                let value = create_memo(move |_| {
                    data.with(|d| d.get(index).map(|e| e.value).unwrap_or(0))
                });
                view! { <p>{value}</p> }
            }
        />
    }
}
