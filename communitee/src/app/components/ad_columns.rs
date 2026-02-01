use leptos::prelude::*;

#[component]
pub fn AdColumns(children: ChildrenFragment) -> impl IntoView {
    view!{
        <div class = "columns">
            <div class ="ads">
            </div>
            <div class = "content">
                {children().nodes}
            </div>
            <div class ="ads">
            </div>
        </div>
    }
}