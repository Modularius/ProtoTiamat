use leptos::prelude::*;

#[component]
pub fn RoundedBox(children: Children) -> impl IntoView {
    view!{
        <div class = "bg-indigo-700 m-4 p-2 rounded-2xl">
            {children()}
        </div>
    }
}

#[component]
pub fn SharpBox(children: Children) -> impl IntoView {
    view!{
        <div class = "bg-indigo-600 m-4 p-2 rounded-2xl">
            {children()}
        </div>
    }
}

#[component]
pub fn ErrorBox(children: Children) -> impl IntoView {
    view!{
        <div class = "bg-red-200 m-4 p-2 rounded-2xl">
            {children()}
        </div>
    }
}