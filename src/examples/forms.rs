use leptos::{html::Input, *};
use web_sys::SubmitEvent;

#[component]
pub fn FormsAndInput() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());
    let input_element: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text" node_ref=input_element value=name/>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}
