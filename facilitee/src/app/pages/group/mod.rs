use abilitee::{
    ContextExt, Expect, TopLevelContext, app::{
        components::{AdColumns, LoginBox, NewPostBox, PostBox, PostData},
        generic_components::{
            ButtonControl, ButtonFunction, ControlStack, ErrorBox, LabelledControlStack, RoundedBox,
        },
        guards::GuardedPage,
    }
};
use leptos::{either::Either, prelude::*};
use leptos_router::{hooks::use_params, params::{Params, ParamsError}};
use libertee::{GroupUuid, SessionUuid, UserUuid};
use serde::{Deserialize, Serialize};
use tracing::instrument;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use abilitee::format_datetime;
        use libertee::{Group, Member, Delegate, UserData};
        use crate::{Server, ServerSideData};
    }
}

#[derive(Clone, Debug, Params, PartialEq)]
pub struct GroupParams {
    group_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupPageDataContext {
    user_id: UserUuid,
    user_name: String,
    group_id: GroupUuid,
    group_name: String,
    member: Option<GroupWithMemberPageData>,
}

impl Expect for GroupPageDataContext {
    const EXPECT: &'static str = "GroupPageDataContext should be provided, this should never fail.";
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupWithMemberPageData {
    datetime_joined: String,
    feed: Vec<PostData>,
    delegates: Vec<Option<GroupWithMemberDelegatePageData>>,
}

#[cfg(feature = "ssr")]
impl GroupWithMemberPageData {
    fn new(server: &Server, group: &Group, member: &Member) -> Self {
        Self {
            datetime_joined: format_datetime(&member.joined),
            feed: group
                .create_feed(&member.id, None, 10)
                .posts
                .into_iter()
                .flat_map(|post| {
                    server
                        .get_user(&post.data.author)
                        .map(|author_user| PostData::new(&post, author_user))
                })
                .collect(),
            delegates: member
                .delegates
                .iter()
                .map(|(delegate_id, &Delegate { weight, .. })| {
                    group
                        .data
                        .members
                        .get(delegate_id)
                        .and_then(|delegate| server.get_user(&delegate.user).ok())
                        .map(|delegate| {
                            GroupWithMemberDelegatePageData::new(&delegate.data, weight)
                        })
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GroupWithMemberDelegatePageData {
    name: String,
    link: String,
    weight: f64,
}

#[cfg(feature = "ssr")]
impl GroupWithMemberDelegatePageData {
    fn new(user_data: &UserData, weight: f64) -> Self {
        Self {
            name: user_data.name.clone(),
            link: format!("/user/{}", user_data.id.0),
            weight,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupPageParamsContext {
    group_id: GroupUuid,
}

impl Expect for GroupPageParamsContext {
    const EXPECT: &'static str =
        "GroupPageParamsContext should be provided, this should never fail.";
}

#[server]
pub async fn get_group_page_data(
    session_id: SessionUuid,
    group_id: GroupUuid,
) -> Result<GroupPageDataContext, ServerFnError> {
    let server_side_data = use_context::<ServerSideData>().expect_context();
    let server = server_side_data.server.lock()?;

    let group = server
        .get_group(&group_id)?;

    let session = server
        .get_session(&session_id)?;

    let user = server
        .get_user(&session.user)?;

    let data = GroupPageDataContext {
        user_id: user.data.id.clone(),
        user_name: user.data.name.clone(),
        group_id: group_id.clone(),
        group_name: group.data.name.clone(),
        member: {
            let member_id = group.get_member_id_from_user_id(&session.user);
            member_id
                .and_then(|member_id| group.data.members.get(&member_id))
                .map(|member| GroupWithMemberPageData::new(&server, group, member))
        },
    };
    Ok(data)
}

pub struct GroupPage;

impl GuardedPage for GroupPage {
    type DataContext = GroupPageDataContext;
    type Source = (usize, usize, Result<GroupParams, ParamsError>);
    
    #[instrument]
    fn source() -> Self::Source {
        let params = use_params::<GroupParams>();
        let top_level_context = use_context::<TopLevelContext>().expect_context();
        (
            top_level_context.login.version().get(),
            top_level_context.logout.version().get(),
            params.get(),
        )
    }

    #[instrument]
    async fn fetch((_, _, params): Self::Source) -> Option<Result<GroupPageDataContext, ServerFnError>> {
        let top_level_context = use_context::<TopLevelContext>()
            .expect_context();
        let session_id = top_level_context.session_id.get_untracked()
            .unwrap().unwrap().unwrap();
        match params {
            Ok(up) => match up.group_id {
                Some(id) => Some(get_group_page_data(session_id, GroupUuid(id)).await),
                None => None,
            },
            Err(_) => None,
        }
    }

    #[instrument]
    fn with_data() -> impl IntoView {
        let group_page_data = use_context::<GroupPageDataContext>().expect_context();
        let member = group_page_data.member;
        view! {
            <h1 class = "text-4xl m-2"> "Hi there " {group_page_data.user_name} "!" </h1>
            <AdColumns>
                <h2 class = "text-2xl m-2"> "Group Name: " {group_page_data.group_name} </h2>
                {
                    if let Some(member) = member{
                        Either::Left(view!{
                            <h3 class = "text-lg m-2"> "You joined on " {member.datetime_joined.clone()} "." </h3>
                            <DelegatePanel delegates = member.delegates.clone()/>
                            <h2 class = "text-lg m-2"> "Submit a post." </h2>
                            <NewPostBox user_id = group_page_data.user_id group_id = Some(group_page_data.group_id) />
                            <RoundedBox>
                                <h2 class = "text-lg m-2"> "Current Group Feed" </h2>
                                <For
                                    each = move ||member.feed.clone().into_iter().enumerate()
                                    key = |(i,_)|*i
                                    children = |(_,post)| view!{ <PostBox post/> }
                                />
                            </RoundedBox>
                        })
                    } else {
                        Either::Right(view!{
                            <ButtonControl value = "Join this group" on_click = ButtonFunction::closure(|_|{}) />
                        })
                    }
                }
            </AdColumns>
        }
    }

    #[instrument]
    fn without_session() -> impl IntoView {
        view! {
            <h1 class = "text-3xl m-6"> "Hi there, welcome to Communitee." </h1>
            <h2 class = "text-xl m-2"> "The social media platform exclusively controlled by its users." </h2>
            <RoundedBox>
                <h3 class = "text-lg m-2"> "Using Communitee guarantees:" </h3>
                <ul class = "text-sm m-2">
                    <li> "Your content and data is *never* used to personalised your feed or the adverts you are shown." </li>
                    <li> "Your experience is curated by yourself and fellow users, and never by an opaque algorithm controlled by tech companies." </li>
                    <li> "You and your fellow users can anonymously vote for the content you like, and this vote exclusively determines which content is shown. There are no paid posts." </li>
                    <li> "All adverts are clearly marked as adverts, and are chosen by the users." </li>
                    <li> "Admins are democratically elected by the users they serve." </li>
                    <li> "Content is moderated by fellow users who are empowered by the democratic wishes of the users they serve." </li>
                    <li> "All users are verified in a safe and anonymous process, which guarantees identity without risking their private data." </li>
                    <li> "Data is distributed among many cooperating nodes, with multiple levels of encryption to ensure privacy." </li>
                </ul>
            </RoundedBox>
            <LoginBox />
        }
    }
}

#[component]
fn DelegatePanel(delegates: Vec<Option<GroupWithMemberDelegatePageData>>) -> impl IntoView {
    view! {
        <RoundedBox>
            <h3 class = "text-lg m-2"> "You have " {delegates.len()} " delegates(s) in this group." </h3>
            <ControlStack>
                <ButtonControl value = "Add New Delegate" on_click = ButtonFunction::closure(|_|{}) />
                <ButtonControl value = "Help on Delegates" on_click = ButtonFunction::Link("/help/delegates") />
            </ControlStack>
            <For
                each=move||delegates.clone().into_iter().enumerate()
                key=|(i,_)|*i
                children=|(_,delegate)| Delegate(DelegateProps{ delegate })
            />
        </RoundedBox>
    }
}

#[component]
fn Delegate(delegate: Option<GroupWithMemberDelegatePageData>) -> impl IntoView {
    if let Some(delegate) = delegate {
        Either::Left(view! {
            <LabelledControlStack label = {format!("{}: {}", delegate.name, delegate.weight)} href = {Some(delegate.link)} class = "w-1/3">
                <ControlStack>
                    <ButtonControl value = "Edit Weight" on_click = ButtonFunction::closure(|_|{}) />
                    <ButtonControl value = "Remove" on_click = ButtonFunction::closure(|_|{}) />
                </ControlStack>
            </LabelledControlStack>
        })
    } else {
        Either::Right(view! {
            <ErrorBox>
                <div> "Delegate not found." </div>
            </ErrorBox>
        })
    }
}
