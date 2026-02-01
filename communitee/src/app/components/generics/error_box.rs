use leptos::prelude::*;

pub fn error_box(errors: ArcRwSignal<Errors>) -> impl IntoView {
    view!{
        <div class = "error-box">
            <For
                each = move || errors.get().into_iter().enumerate()
                key = |(i,_)|*i
                let((idx,err))
            >
                <div> "Error " {idx} "| " {format!("{err:?}")} </div>
            </For>
        </div>
    }
}