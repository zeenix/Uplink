use common::icons::outline::Shape as Icon;
use common::icons::Icon as IconElement;
use common::language::get_local_text;
use common::state::{identity_search_result, Chat, Identity, State};
use dioxus::prelude::*;
use kit::components::{user_image::UserImage, user_image_group::UserImageGroup};

use warp::crypto::DID;

use crate::utils::build_participants;

#[derive(Props)]
pub struct SearchProps<'a> {
    search_typed_chars: UseRef<String>,
    search_friends_is_focused: UseRef<bool>,
    search_dropdown_hover: UseRef<bool>,
    identities: UseState<Vec<identity_search_result::Entry>>,
    friends_identities: UseState<Vec<Identity>>,
    chats: UseState<Vec<Chat>>,
    onclick: EventHandler<'a, identity_search_result::Identifier>,
}

pub fn search_friends<'a>(cx: Scope<'a, SearchProps<'a>>) -> Element<'a> {
    let state = use_shared_state::<State>(cx)?;
    if cx.props.identities.get().is_empty() || !*cx.props.search_friends_is_focused.read() {
        return None;
    }

    let mut friends_identities = cx.props.friends_identities.get().clone();
    let chats = cx.props.chats.get().clone();

    friends_identities.sort_by_key(|identity| identity.username());

    cx.render(rsx!(
        div {
            class: "searchbar-dropdown",
            aria_label: "searchbar-dropwdown",
            onblur: |_| {
                *cx.props.search_friends_is_focused.write() = false;
            },
            onmouseenter: |_| {
                *cx.props.search_dropdown_hover.write_silent() = true;
            },
            onmouseleave: |_| {
                *cx.props.search_dropdown_hover.write_silent() = false;
            },
            if !friends_identities.is_empty() {
                rsx!(
                    div {
                        id: "users-searchdropdown-label",
                        class: "users-groups-label",
                        aria_label: "users-groups-label",
                        p {
                            get_local_text("uplink.users")
                        }
                    })
            }
            friends_identities.iter().cloned().map(|identity| {
                let username = identity.username();
                let did = identity.did_key();
                let did2 = did.clone();
                let search_typed_chars = cx.props.search_typed_chars.read().clone();
                let start = username.to_lowercase().find(&search_typed_chars.to_lowercase()).unwrap_or(0);
                let end = start + search_typed_chars.len();
                let blocked_friends: Vec<DID> = state
                    .read()
                    .blocked_fr_identities()
                    .iter()
                    .map(|f| f.did_key())
                    .collect();

                rsx!(
                    div {
                        class: "identity-header-sidebar",
                        aria_label: "search-result-user",
                        opacity: format_args!("{}", if blocked_friends.contains(&did2) {"0.5"} else {"1"}),
                        prevent_default: "onclick",
                        onclick: move |evt| {
                            if !blocked_friends.contains(&did2) {
                                evt.stop_propagation();
                                *cx.props.search_friends_is_focused.write_silent() = false;
                                cx.props.onclick.call(identity_search_result::Identifier::Did(did.clone()));
                            }
                        },
                        UserImage {
                            platform: identity.platform().into(),
                            status: identity.identity_status().into(),
                            image: identity.profile_picture()
                        },
                        div {
                            class: "search-friends-dropdown-name",
                            aria_label: "search-friends-dropdown-name",
                            rsx!(
                                span { &username[0..start] },
                                span {
                                    class: "highlight-search-typed-chars",
                                    aria_label: "highlight-search-typed-chars",
                                    &username[start..end]
                                },
                                span {
                                    aria_label: "remaining-match-search",
                                    &username[end..]
                                },
                            )
                        }
                        if blocked_friends.contains(&did2) {
                            rsx!(
                                div {
                                    padding_right: "32px",
                                    aria_label: "search-result-blocked-user",
                                    display: "flex",
                                    IconElement {
                                        size: 40,
                                        fill: "var(--text-color-muted)",
                                        icon: Icon::UserBlocked,
                                    },
                                }
                            )
                        }
                    }
                )
            })
            if !chats.is_empty() && !friends_identities.is_empty() {
                rsx!(div { class:"border", })
            }
            if !chats.is_empty() {
                rsx!(
                    div {
                        id: "groups-searchdropdown-label",
                        class: "users-groups-label",
                        aria_label: "users-groups-label",
                        p {
                            get_local_text("uplink.groups")
                        }
                    }
                )
            }
            chats.iter().cloned().map(|chat| {
                let id = chat.id;
                let participants = state.read().chat_participants(&chat);
                let participants2 = participants.clone();

                let other_participants_names = State::join_usernames(&participants);
                let conversation_title = match chat.conversation_name.as_ref() {
                    Some(n) => n.clone(),
                    None => other_participants_names,
                };
                let search_typed_chars = cx.props.search_typed_chars.read().clone();
                let text_to_find = search_typed_chars.to_lowercase();
                let search_typed_chars2 = search_typed_chars.clone();

                rsx!(
                    div {
                        class: "identity-header-sidebar",
                        aria_label: "search-result-group",
                        prevent_default: "onclick",
                        onclick: move |evt|  {
                                evt.stop_propagation();
                                *cx.props.search_friends_is_focused.write_silent() = false;
                                cx.props.onclick.call(identity_search_result::Identifier::Uuid(id));
                        },
                        rsx! (
                            div {
                                class: "user-image-group",
                                div {
                                    aria_label: "user-image-group-wrap",
                                    class: "user-image-group-wrap group",
                                    rsx!(
                                        UserImageGroup {
                                            loading: false,
                                            aria_label: "user-image-group".into(),
                                            participants: build_participants(&participants),
                                        }
                                    )
                                },
                            }
                        div {
                                class: "search-friends-dropdown-name",
                                aria_label: "search-friends-dropdown-name",
                                if let Some(start) = conversation_title.to_lowercase().find(&text_to_find) {
                                    let end = start + search_typed_chars2.len();
                                    rsx!(
                                        span { &conversation_title[0..start] },
                                        span {
                                            class: "highlight-search-typed-chars",
                                            aria_label: "highlight-search-typed-chars",
                                            &conversation_title[start..end]
                                        },
                                        span {
                                            aria_label: "remaining-match-search",
                                            &conversation_title[end..]
                                        },
                                    )
                                } else {
                                    rsx!(span { conversation_title})
                                }
                            }
                        )
                    }
                    if !participants2.is_empty() &&
                    participants2.iter().any(|identity| identity.username().to_lowercase().starts_with(&search_typed_chars.to_lowercase())
                    &&
                    identity.did_key() != state.read().did_key()
                ) {
                        rsx!(
                            div {
                                id: "members-searchdropdown-label",
                                aria_label: "members-searchdropdown-label",
                                padding_left: "48px",
                                padding_top: "4px",
                                p {
                                    color: "var(--text-color)",
                                    font_size: "12px",
                                    get_local_text("uplink.members")
                                }
                            }
                        )
                    },
                    participants2.iter().cloned()
                    .filter(|identity| identity.username().to_lowercase().starts_with(&search_typed_chars.to_lowercase())
                    &&
                    identity.did_key() != state.read().did_key()
                )
                    .map(|identity| {
                        let typed_chars = search_typed_chars.clone();
                        let username = identity.username();
                        let did = identity.did_key();
                        let did2 = did.clone();
                        let start = username.to_lowercase().find(&typed_chars.to_lowercase()).unwrap_or(0);
                        let end = start + typed_chars.len();
                        let blocked_friends: Vec<DID> = state
                        .read()
                        .blocked_fr_identities()
                        .iter()
                        .map(|f| f.did_key())
                        .collect();


                        rsx!(
                            div {
                                class: "identity-header-sidebar-participants-in-group",
                                opacity: format_args!("{}", if blocked_friends.contains(&did2) {"0.5"} else {"1"}),
                                aria_label: "search-result-participant-in-group",
                                prevent_default: "onclick",
                                onclick: move |evt| {
                                    if !blocked_friends.contains(&did2) {
                                        evt.stop_propagation();
                                        *cx.props.search_friends_is_focused.write_silent() = false;
                                        cx.props.onclick.call(identity_search_result::Identifier::Did(did.clone()));
                                    }
                                },
                                UserImage {
                                    platform: identity.platform().into(),
                                    status: identity.identity_status().into(),
                                    image: identity.profile_picture()
                                },
                                div {
                                    class: "search-friends-dropdown-name",
                                    aria_label: "search-friends-dropdown-name",
                                    rsx!(
                                        span { &username[0..start] },
                                        span {
                                            class: "highlight-search-typed-chars",
                                            aria_label: "highlight-search-typed-chars",
                                            &username[start..end]
                                        },
                                        span {
                                            aria_label: "remaining-match-search",
                                            &username[end..]
                                        },
                                    )
                                }
                                if blocked_friends.contains(&did2) {
                                    rsx!(
                                        div {
                                            padding_right: "32px",
                                            display: "flex",
                                            aria_label: "search-result-blocked-user-in-group",
                                            IconElement {
                                                size: 40,
                                                fill: "var(--text-color-muted)",
                                                icon: Icon::UserBlocked,
                                            },
                                        }
                                    )
                                }
                            }
                        )
                    })
                )
            })
        }
    ))
}
