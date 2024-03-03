use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
  let (first, set_first) =
    create_signal("Micky".to_string());
  let (last, set_last) =
    create_signal("Hallabrin".to_string());
  let (value, set_value) =
    create_signal("B".to_string());
  let input_element: NodeRef<html::Input> =
    create_node_ref();
  let on_submit =
    move |ev: leptos::ev::SubmitEvent| {
      // stop the page from reloading!
      ev.prevent_default();

      // here, we'll extract the value from the input
      let value = input_element()
        // event handlers can only fire after the view
        // is mounted to the DOM, so the `NodeRef` will be `Some`
        .expect("<input> should be mounted")
        // `leptos::HtmlElement<html::Input>` implements `Deref`
        // to a `web_sys::HtmlInputElement`.
        // this means we can call`HtmlInputElement::value()`
        // to get the current value of the input
        .value();
      set_last(value);
    };

  view! {
      <input
          type="text"
          on:input=move |ev| {
              set_first(event_target_value(&ev));
          }

          // the `prop:` syntax lets you update a DOM property,
          // rather than an attribute.
          prop:value=first
      />
      <select on:change=move |ev| {
          let new_value = event_target_value(&ev);
          set_value(new_value);
      }>
          <SelectOption value is="A"/>
          <SelectOption value is="B"/>
          <SelectOption value is="C"/>
      </select>
      <p>"Name is: " {first}</p>
      // on_submit defined below
      <form on:submit=on_submit>
          <input type="text" value=last node_ref=input_element/>
          <input type="submit" value="Submit"/>
      </form>
      <p>"Name is: " {last}</p>
  }
}

#[allow(non_snake_case)]
#[component]
pub fn SelectOption(
  is: &'static str,
  value: ReadSignal<String>,
) -> impl IntoView {
  view! {
      <option value=is selected=move || value() == is>
          {is}
      </option>
  }
}
