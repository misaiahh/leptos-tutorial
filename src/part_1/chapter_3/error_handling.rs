use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn NumericInput() -> impl IntoView {
  let (value, set_value) = create_signal(Ok(0));

  let on_input = move |ev| {
    set_value(
      event_target_value(&ev).parse::<i32>(),
    )
  };

  view! {
      <label>
          "Type a number (or not!)" <input type="number" on:input=on_input/>
          <ErrorBoundary fallback=|errors| {
              view! {
                  <div class="error">
                      <p>"Not a number!"</p>
                      <ul>
                          {move || {
                              errors
                                  .get()
                                  .into_iter()
                                  .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                  .collect_view()
                          }}

                      </ul>
                  </div>
              }
          }>

              <p>"You entered " <strong>{value}</strong></p>
          </ErrorBoundary>
      </label>
  }
}
