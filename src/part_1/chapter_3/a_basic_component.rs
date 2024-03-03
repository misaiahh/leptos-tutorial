use leptos::*;

#[component]
pub fn app() -> impl IntoView {
  let (count, set_count) = create_signal(0);

  view! {
      <button on:click=move |_| {
          set_count.update(|n| *n += 1);
      }>"Click me: " {count}</button>
  }
}
