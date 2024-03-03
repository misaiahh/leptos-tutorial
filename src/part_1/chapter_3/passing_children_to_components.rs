use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn TakesChildren<F, IV>(
  /// Takes a function (type F) that returns anything that can be
  /// converted into a View (type IV)
  render_prop: F,
  /// `children` takes the `Children` type
  children: Children,
) -> impl IntoView
where
  F: Fn() -> IV,
  IV: IntoView,
{
  view! {
      <h2>"Render Prop"</h2>
      {render_prop()}

      <h2>"Children"</h2>
      {children()}
  }
}

#[allow(non_snake_case)]
#[component]
pub fn WrapsChildren(
  children: Children,
) -> impl IntoView {
  // Fragment has `nodes` field that contains a Vec<View>
  let children = children()
    .nodes
    .into_iter()
    .map(|child| view! { <li>{child}</li> })
    .collect_view();

  view! { <ul>{children}</ul> }
}
