use crate::{
    app::{TopLevelContext, generic_components::{
        ControlStack, ErrorBox, LabelledInput, LabelledSelect, SubmitControl
    }},
    server_functions::{PerformLogin, PerformLogout, Register}, structs::ContextExt,
};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[component]
pub fn LoginBox(
    #[prop(optional)]
    redirect_to: Option<&'static str>
) -> impl IntoView {
    let login = ServerAction::<PerformLogin>::new();
    Effect::new(move || {
        if let Some(Ok(_session)) = login.value().get() {
            let top_level_context = use_context::<TopLevelContext>()
                .expect_context();
            top_level_context.session.refetch();
        }
    });
    view! {
        <ActionForm action = login>
            {redirect_to.map(|redirect_to|view!{
                <input type = "hidden" name = "redirect_to" value = {redirect_to}/>
            })}
            
            <Show when = move ||login.value().get().is_some_and(|value|value.is_err())>
                <ErrorBox>
                    "Login Failed."
                </ErrorBox>
            </Show>
            <ControlStack>
                <LabelledInput name = "auth[username]" label = "Username" typ = "text" value = ""/>
                <LabelledInput name = "auth[password]" label = "password" typ = "password" value = ""/>
            </ControlStack>
            <ControlStack>
                <SubmitControl value = "Login" />
            </ControlStack>
        </ActionForm>
    }
}

#[component]
pub fn LogoutBox(
    #[prop(optional)]
    redirect_to: Option<&'static str>
) -> impl IntoView {
    let logout = ServerAction::<PerformLogout>::new();
    Effect::new(move || {
        if let Some(Ok(true)) = logout.value().get() {
            let top_level_context = use_context::<TopLevelContext>()
                .expect_context();
            top_level_context.session.refetch();
        }
    });
    view! {
        <ActionForm action = logout>
            {redirect_to.map(|redirect_to|view!{
                <input type = "hidden" name = "redirect_to" value = {redirect_to}/>
            })}
            
            <Show when = move ||logout.value().get().is_some_and(|value|value.is_err() || value.is_ok_and(|value|!value))>
                <ErrorBox>
                    "Logout Failed."
                </ErrorBox>
            </Show>
            <ControlStack>
                <SubmitControl value = "Logout" />
            </ControlStack>
        </ActionForm>
    }
}

#[derive(
    Default,
    Clone,
    Debug,
    EnumString,
    Display,
    EnumIter,
    PartialEq,
    Eq,
    Hash,
    Copy,
    Serialize,
    Deserialize,
)]
pub enum ProofOfIdentity {
    #[default]
    #[strum(to_string = "Approval of Existing Member(s)")]
    ExistingUser,
    #[strum(to_string = "Birth Certificate")]
    BirthCertificate,
    #[strum(to_string = "Driving Licence")]
    DrivingLicence,
}

#[component]
pub fn RegisterBox() -> impl IntoView {
    let register = ServerAction::<Register>::new();
    let proof_of_identity = RwSignal::<ProofOfIdentity>::new(ProofOfIdentity::default());
    view! {
        <ActionForm action = register>
            <ControlStack>
                <LabelledInput name = "username" label = "Username" typ = "text" value = ""/>
                <LabelledInput name = "password" label = "Password" typ = "password" value = ""/>
            </ControlStack>
            <ControlStack>
                <LabelledSelect name = "proof_of_identity" label = "Proof of Identity" sig = {proof_of_identity}/>
            </ControlStack>
            <ControlStack>
                { move ||
                    match proof_of_identity.get() {
                        ProofOfIdentity::ExistingUser => view!{
                            <LabelledInput name = "approver" label = "Name of Approver" typ = "text" value = ""/>
                        }.into_any(),
                        ProofOfIdentity::BirthCertificate => view!{
                            <LabelledInput name = "birth_certificate_number" label = "Birth Certificate Serial Number" typ = "text" value = ""/>
                        }.into_any(),
                        ProofOfIdentity::DrivingLicence => view!{
                            <LabelledInput name = "driving_licence_number" label = "Driving Licence Serial Number" typ = "text" value = ""/>
                        }.into_any(),
                    }
                }
            </ControlStack>
            <ControlStack>
                <SubmitControl value = "Register" />
            </ControlStack>
        </ActionForm>
    }
}
