use leptos::{ev::MouseEvent, prelude::*};

use crate::app::generic_components::Control;

#[component]
pub fn ButtonControl<F>(
    value: &'static str,
    on_click: F
) -> impl IntoView
 where F: FnMut(MouseEvent) + Send + 'static {
    view! {
        <Control>
            <input class = "flex-grow" type = "button" value = {value} on:click = {on_click} />
        </Control>
    }
}

#[component]
pub fn SubmitControl(
    value: &'static str
) -> impl IntoView {
    view! {
        <Control>
            <input class = "flex-grow" type = "submit" value = {value} />
        </Control>
    }
}

#[component]
pub fn CloseButton() -> impl IntoView {
    view!{
        <div class = "w-full rounded-sm h-4 bg-indigo-100 hover:bg-blue-100" on:click:target = |ev| {
            if let Some(el) = ev.target().parent_element() {
                match el.class_list().toggle("closed") {
                    Ok(_js) => {},
                    Err(_e) => {}
                }
            }
        }>
        </div>
    }
}