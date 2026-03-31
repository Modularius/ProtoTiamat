use leptos::prelude::*;
use tracing::instrument;

#[component]
#[instrument(skip_all)]
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
#[instrument(skip_all)]
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
