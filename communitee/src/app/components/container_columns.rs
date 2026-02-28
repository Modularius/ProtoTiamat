use leptos::prelude::*;

#[component]
pub fn MainColumn(children: Children) -> impl IntoView {
    view! {
        <div class = "middle-column">
            <div class = "middle-column-inner">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn AdColumns(children: Children) -> impl IntoView {
    view! {
        <div class = "columns">
            <div class ="ads">
            </div>
            <div class = "content">
                {children()}
            </div>
            <div class ="ads">
            </div>
        </div>
    }
}
