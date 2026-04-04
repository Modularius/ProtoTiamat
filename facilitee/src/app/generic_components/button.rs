use leptos::{either::Either, ev::MouseEvent, prelude::*};
use leptos_router::components::A;

use crate::app::generic_components::Control;

pub enum ButtonFunction {
    Link(&'static str),
    Closure(Box<dyn FnMut(MouseEvent) + Send + 'static>),
}

impl ButtonFunction {
    pub fn closure<F: FnMut(MouseEvent) + Send + 'static>(f: F) -> Self {
        Self::Closure(Box::new(f))
    }
}

#[component]
pub fn ButtonControl(value: &'static str, on_click: ButtonFunction) -> impl IntoView {
    match on_click {
        ButtonFunction::Link(href) => Either::Left(view! {
            <Control>
                <A attr:class = "flex-grow" href = {href}>
                    {value}
                </A>
            </Control>
        }),
        ButtonFunction::Closure(on_click) => Either::Right(view! {
            <Control>
                <input class = "flex-grow" type = "button" value = {value} on:click = {on_click} />
            </Control>
        }),
    }
}

#[component]
pub fn SubmitControl(value: &'static str) -> impl IntoView {
    view! {
        <Control>
            <input class = "flex-grow" type = "submit" value = {value} />
        </Control>
    }
}

#[component]
pub fn CloseButton() -> impl IntoView {
    view! {
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
