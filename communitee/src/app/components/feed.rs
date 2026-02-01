use crate::app::components::post::{PostBox, PostBoxProps};
use leptos::prelude::*;
/*
#[component]
pub fn Feed<F>(mut feed: F, max: usize) -> impl IntoView where F: FeedSource {
    let posts = (0..max).flat_map(|_|feed.next_post()).collect::<Vec<_>>();
    view!{
        <For
            each = move ||posts.clone().into_iter().enumerate()
            key = |(i,_)|*i
            children = move |(_,post)| PostBox(PostBoxProps { post })
        />
    }
}
    */