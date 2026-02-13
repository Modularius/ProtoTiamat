use leptos::{ev::{Event, MouseEvent, Targeted}, html::{ElementType, Select}, prelude::*, tachys::html::property::IntoProperty};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use crate::app::generic_components::Control;
use std::{hash::Hash, str::FromStr, fmt::Debug};
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
pub fn LabelledSelect<T>(
    name: &'static str,
    label: &'static str,
    sig: RwSignal<T>
) -> impl IntoView where 
 T: Clone + ToString + IntoEnumIterator + PartialEq + Eq + Hash + Send + 'static,
 <T as IntoEnumIterator>::Iterator : Send,
 <RwSignal<T> as Update>::Value: FromStr,
 <<RwSignal<T> as Update>::Value as FromStr>::Err : Debug,
 <RwSignal<T> as With>::Value: Clone + PartialEq<T>,
 RwSignal<T>: Update + With {
    view! {
        <Control>
            <div class = "text-left justify-between">
                <label class = "" for = {name}>
                    {label}
                </label>
                <select name = {name} id = {name} class = ""
                    on:change = move |ev|
                        sig.set(
                            event_target_value(&ev)
                                .parse()
                                .expect("SearchMode value should parse, this should never fail.")
                            )>
                    <For each = T::iter
                        key = ToOwned::to_owned
                        let(poi)
                    >
                        <option selected={sig.get() == poi} value = {poi.to_string()}> {poi.to_string()} </option>
                    </For>
                </select>
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
/*
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
 */