use leptos::prelude::*;

#[component]
pub fn ControlStack(children: Children) -> impl IntoView {
    view! {
        <div class = "flex flex-grow
            flex-col lg:flex-row
            p-1 md:p-2
            m-1 md:m-2
            space-x-1 lg:space-x-2
            space-y-1 lg:space-y-2
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
            bg-blue-700 hover:bg-blue-800
            text-center">
            {children()}
        </div>
    }
}