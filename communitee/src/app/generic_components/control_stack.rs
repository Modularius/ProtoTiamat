use leptos::{either::Either, prelude::*};
use leptos_router::components::A;

#[component]
pub fn LabelledControlStack(label: String, href: Option<String>, class: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class = "text-purple-100 flex flex-shrink">
            <div class = {format!("flex flex-shrink items-center justify-left p-1 pl-4 m-1 {class}")}>
                {if let Some(href) = href {
                    Either::Left(view!{<A href = {href}> {label.clone()} </A>})
                } else {
                    Either::Right(label.clone())
                }}
            </div>
            <ControlStack>
                {children()}
            </ControlStack>
        </div>
    }
}

#[component]
pub fn ControlStack(children: Children) -> impl IntoView {
    view! {
        <div class = "flex flex-grow
            flex-col lg:flex-row
            p-1 md:p-2
            m-1 md:m-2
            space-x-0 lg:space-x-2
            space-y-1 lg:space-y-0
            bg-blue-600">
            {children()}
        </div>
    }
}

#[component]
pub fn ControlStackRow(children: Children) -> impl IntoView {
    view! {
        <div class = "flex flex-grow flex-col
            p-1 md:p-2
            m-1 md:m-2
            space-x-0 lg:space-x-2
            space-y-1 lg:space-y-0
            bg-blue-600">
            {children()}
        </div>
    }
}

#[component]
pub fn Control(children: Children) -> impl IntoView {
    view! {
        <div class = "flex flex-grow
            flex-col lg:flex-row
            p-1 lg:p-2
            bg-blue-700 hover:bg-blue-500
            text-center">
            {children()}
        </div>
    }
}