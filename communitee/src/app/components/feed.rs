use crate::app::components::{
    PostData,
    post::{PostBox, PostBoxProps},
};
use leptos::prelude::*;
use tracing::instrument;

#[component]
#[instrument(skip_all)]
pub fn Feed<F>(_feed: F, _max: usize) -> impl IntoView
where
    F: Iterator<Item = PostData>,
{
    let posts = _feed.take(_max).collect::<Vec<_>>();
    view! {
        <For
            each = move ||posts.clone().into_iter().enumerate()
            key = |(i,_)|*i
            children = move |(_,post)| PostBox(PostBoxProps { post })
        />
    }
}
