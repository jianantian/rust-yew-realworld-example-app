use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

use crate::agent::Profiles;
use crate::components::article_list::{ArticleList, ArticleListFilter};
use crate::error::Error;
use crate::types::{ProfileInfo, ProfileInfoWrapper, UserInfo};

/// Profile for an author
pub struct Profile {
    profiles: Profiles,
    profile: Option<ProfileInfo>,
    response: Callback<Result<ProfileInfoWrapper, Error>>,
    task: Option<FetchTask>,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub username: String,
    #[props(required)]
    pub current_user: Option<UserInfo>,
    #[props(required)]
    pub tab: ProfileTab,
}

#[derive(Clone)]
pub enum Msg {
    Response(Result<ProfileInfoWrapper, Error>),
    Follow,
    UnFollow,
}

#[derive(Clone, PartialEq)]
pub enum ProfileTab {
    ByAuthor,
    FavoritedBy,
}

impl Component for Profile {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Profile {
            profiles: Profiles::new(),
            profile: None,
            response: link.send_back(Msg::Response),
            task: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.task = Some(
            self.profiles
                .get(self.props.username.clone(), self.response.clone()),
        );
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Follow => {
                self.task = Some(
                    self.profiles
                        .follow(self.props.username.clone(), self.response.clone()),
                );
            }
            Msg::UnFollow => {
                self.task = Some(
                    self.profiles
                        .unfollow(self.props.username.clone(), self.response.clone()),
                );
            }
            Msg::Response(Ok(profile_info)) => {
                self.profile = Some(profile_info.profile);
                self.task = None;
            }
            Msg::Response(Err(_)) => {
                self.task = None;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html<Self> {
        let is_current_user = if let Some(current_user) = &self.props.current_user {
            current_user.username == self.props.username
        } else {
            false
        };

        if let Some(profile) = &self.profile {
            html! {
                <div class="profile-page">
                    <div class="user-info">
                        <div class="container">
                            <div class="row">
                                <div class="col-xs-12 col-md-10 offset-md-1">
                                    <img src={ &profile.image } class="user-img" alt={ &profile.username } />
                                    <h4>{ &profile.username }</h4>
                                    <p>
                                        {
                                            if let Some(bio) = &profile.bio {
                                                html! { bio }
                                            } else {
                                                html! { }
                                        }}
                                    </p>
                                    {
                                        if is_current_user {
                                            self.view_edit_profile_settings()
                                        } else {
                                            self.view_follow_user_button()
                                    }}
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="container">
                        <div class="row">
                            <div class="col-xs-12 col-md-10 offset-md-1">
                                <div class="articles-toggle">
                                    { self.view_tabs() }
                                </div>
                                {
                                    match self.props.tab {
                                        ProfileTab::ByAuthor => {
                                            html! { <ArticleList filter=ArticleListFilter::ByAuthor(profile.username.clone()) /> }
                                        }
                                        ProfileTab::FavoritedBy => {
                                            html! { <ArticleList filter=ArticleListFilter::FavoritedBy(profile.username.clone()) /> }
                                        }
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }
}

impl Profile {
    fn view_edit_profile_settings(&self) -> Html<Self> {
        html! {
            <RouterLink
                link={ format!("#/settings") }
                classes="btn btn-sm btn-outline-secondary action-btn"
                text={ "Edit Profile Settings" } />
        }
    }

    fn view_follow_user_button(&self) -> Html<Self> {
        if let Some(profile) = &self.profile {
            let class = if profile.following {
                "btn btn-sm action-btn btn-secondary"
            } else {
                "btn btn-sm action-btn btn-outline-secondary"
            };

            let onclick = if profile.following {
                Msg::UnFollow
            } else {
                Msg::Follow
            };

            let text = if profile.following {
                "Unfollow"
            } else {
                "Follow"
            };

            html! {
                <button
                    class=class
                    onclick=|_| onclick.clone() >
                    { text }
                </button>
            }
        } else {
            html! {}
        }
    }

    fn view_tabs(&self) -> Html<Self> {
        if let Some(profile) = &self.profile {
            let classes = if self.props.tab == ProfileTab::ByAuthor {
                ("nav-link active", "nav-link")
            } else {
                ("nav-link", "nav-link active")
            };

            html! {
                <ul class="nav nav-pills outline-active">
                    <li class="nav-item">
                        <RouterLink
                            classes=classes.0
                            link={ format!("#/@{}", &profile.username) }
                            text={ "My Articles" } />
                    </li>
                    <li className="nav-item">
                        <RouterLink
                            classes=classes.1
                            link={ format!("#/@{}/favorites", &profile.username) }
                            text={ "Favorited Articles" } />
                    </li>
                </ul>
            }
        } else {
            html! {}
        }
    }
}
