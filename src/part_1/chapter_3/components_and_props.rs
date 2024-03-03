use leptos::*;

// #[allow(non_snake_case)]
// #[component]
// fn ProgressBar<F: Fn() -> i32 + 'static>(
//     // #[prop(optional)]
//     #[prop(default = 100)] max: u16,
//     // progress: ReadSignal<i32>,
//     progress: F,
// ) -> impl IntoView
// // where
// //     F: Fn() -> i32 + 'static,
// {
//     view! { <progress max=max value=progress></progress> }
// }

#[allow(non_snake_case)]
#[component]
fn ProgressBar(
  #[prop(default = 100)] max: u16,
  #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
  view! { <progress max=max value=progress></progress> }
}

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
  let (count, set_count) = create_signal(0);
  let double_count =
    Signal::derive(move || count() * 2);
  view! {
      <button on:click=move |_| {
          set_count.update(|n| *n += 1);
      }>"Click me"</button>
      <ProgressBar progress=count/>
      <ProgressBar progress=double_count/>
  }
}
