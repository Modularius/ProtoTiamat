use leptos::prelude::*;

#[component]
pub fn FootBar() -> impl IntoView {
    view!{
        <div class = "bottom-bar">
            <div class = "bottom-bar-inner">
                <div class = "tagline"> "The Internet we were Promised" </div>
                <div class = "title"> "Communitee" </div>
                <div class = "right-bar">
                </div>
            </div>
        </div>
    }
}