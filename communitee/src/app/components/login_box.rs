use crate::{app::generic_components::{Control, ControlStack, LabelledInput, LabelledSelect, SubmitControl}, server_functions::{PerformLogin, Register}};
use leptos::{attr::Default, prelude::*};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[component]
pub fn LoginBox() -> impl IntoView {
    let login = ServerAction::<PerformLogin>::new();
    view! {
        <ActionForm action = login>
            <ControlStack>
                <LabelledInput name = "username" label = "Username" typ = "text" value = ""/>
                <LabelledInput name = "password" label = "password" typ = "password" value = ""/>
            </ControlStack>
            <ControlStack>
                <SubmitControl value = "Login" />
            </ControlStack>
        </ActionForm>
    }
}

#[derive(Default, Clone, Debug, EnumString, Display, EnumIter, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub enum ProofOfIdentity {
    #[default]
    #[strum(to_string = "Approval of Existing Member(s)")]
    ExistingUser,
    #[strum(to_string = "Birth Certificate")]
    BirthCertificate,
    #[strum(to_string = "Driving Licence")]
    DrivingLicence
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
