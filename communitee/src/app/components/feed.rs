use crate::app::components::{
    PostData,
    post::{PostBox, PostBoxProps},
};
use leptos::prelude::*;

#[component]
pub fn Feed<F>(feed: F, max: usize) -> impl IntoView
where
    F: Iterator<Item = PostData>,
{
    let posts = feed.take(max).collect::<Vec<_>>();
    view! {
        <For
            each = move ||posts.clone().into_iter().enumerate()
            key = |(i,_)|*i
            children = move |(_,post)| PostBox(PostBoxProps { post })
        />
    }
}
