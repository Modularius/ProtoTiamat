use leptos::prelude::*;


#[component]
pub fn MainColumn(children: ChildrenFragment) -> impl IntoView {
    view!{
        <div class = "middle-column">
            <div class = "middle-column-inner">
                {children().nodes}
            </div>
        </div>
    }
}