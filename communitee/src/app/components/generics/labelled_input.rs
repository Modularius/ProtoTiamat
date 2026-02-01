use leptos::prelude::*;


#[component]
pub fn LabelledInput(name: &'static str, label: &'static str, typ: &'static str, value: &'static str) -> impl IntoView {
    view!{
        <div class = "control-layer">
            <label> {label} </label>
            <input name = {name} type = {typ} value = {value} />
        </div>
    }
}

#[component]
pub fn BoundLabelledInput(name: &'static str, label: &'static str, typ: &'static str, signal: RwSignal<String>) -> impl IntoView {
    view!{
        <div class = "control-layer">
            <label for = {name}> {label} </label>
            <input name = {name} id = {name} type = {typ} bind:value = signal />
        </div>
    }
}