use leptos::*;
use web_sys::MouseEvent;

// adding my own custom struct to the context :)
#[derive(Copy, Clone)]
struct ToggledSetter(WriteSignal<bool>);

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
  let (toggled, set_toggled) =
    create_signal(false);
  // used by ButtonD in #4
  provide_context(ToggledSetter(set_toggled));

  view! {
      <p>"Toggled? " {toggled}</p>
      // <ButtonA setter=set_toggled/>
      // <ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>
      // <ButtonB2 on_click=move |_| set_toggled.update(|value| *value = !*value)/>
      // <ButtonC on:click=move |_| set_toggled.update(|value| *value = !*value)/>
      <Layout/>
  }
}

// 1. Pass a WriteSignal

#[allow(non_snake_case)]
#[component]
pub fn ButtonA(
  setter: WriteSignal<bool>,
) -> impl IntoView {
  view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle"</button> }
}

// 2. Use a Callback

#[allow(non_snake_case)]
#[component]
pub fn ButtonB(
  #[prop(into)] on_click: Callback<MouseEvent>,
) -> impl IntoView {
  view! { <button on:click=on_click>"Toggle"</button> }
}

// 2.1 Use Closure instead of Callback

#[allow(non_snake_case)]
#[component]
pub fn ButtonB2<F>(on_click: F) -> impl IntoView
where
  F: Fn(MouseEvent) + 'static,
{
  view! { <button on:click=on_click>"Toggle"</button> }
}

// 3. Use an Event Listener

#[allow(non_snake_case)]
#[component]
pub fn ButtonC() -> impl IntoView {
  view! { <button>"Toggle"</button> }
}

// 4. Providing a Context

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
#[component]
pub fn Content() -> impl IntoView {
  view! {
      <div class="content">
          <ButtonD/>
      </div>
  }
}

#[allow(non_snake_case)]
#[component]
pub fn ButtonD() -> impl IntoView {
  // use_context searches up the context tree, hoping to
  // find a `WriteSignal<bool>`
  // in this case, I .expect() because I know I provided it
  let setter =
    use_context::<ToggledSetter>().unwrap().0;

  view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle"</button> }
}
