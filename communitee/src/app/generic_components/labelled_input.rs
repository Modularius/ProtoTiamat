use leptos::prelude::*;

use crate::app::generic_components::Control;

#[component]
pub fn LabelledInput(
    name: &'static str,
    label: &'static str,
    typ: &'static str,
    value: &'static str,
) -> impl IntoView {
    view! {
        <Control>
            <div class = "text-left justify-between">
                <label for = {name}> {label} </label>
                <input class = "italic" name = {name} type = {typ} value = {value} />
            </div>
        </Control>
    }
}

#[component]
pub fn LabelledTextArea(
    name: &'static str,
    label: &'static str,
    value: &'static str,
) -> impl IntoView {
    view! {
        <Control>
            <div class = "text-left flex flex-col w-full">
                <label class = "w-full" for = {name}> {label} </label>
                <textarea class = "w-full italic" name = {name}>{value}</textarea>
            </div>
        </Control>
    }
}

#[component]
pub fn BoundLabelledInput(
    name: &'static str,
    label: &'static str,
    typ: &'static str,
    signal: RwSignal<String>,
) -> impl IntoView {
    view! {
        <div class = "control-layer">
            <label for = {name}> {label} </label>
            <input name = {name} id = {name} type = {typ} bind:value = signal />
        </div>
    }
}
