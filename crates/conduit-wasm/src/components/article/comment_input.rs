use stdweb::web::event::IEvent;
use yew::services::fetch::FetchTask;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::agent::Comments;
use crate::components::list_errors::ListErrors;
use crate::error::Error;
use crate::types::{
    CommentCreateInfo, CommentCreateInfoWrapper, CommentInfo, CommentInfoWrapper, UserInfo,
};

/// Creat a comment for an article.
pub struct CommentInput {
    comments: Comments,
    error: Option<Error>,
    request: CommentCreateInfo,
    response: Callback<Result<CommentInfoWrapper, Error>>,
    task: Option<FetchTask>,
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub slug: String,
    #[props(required)]
    pub current_user: UserInfo,
    #[props(required)]
    pub callback: Callback<CommentInfo>,
}

pub enum Msg {
    Request,
    Response(Result<CommentInfoWrapper, Error>),
    UpdateComment(String),
}

impl Component for CommentInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        CommentInput {
            error: None,
            comments: Comments::new(),
            request: CommentCreateInfo::default(),
            response: link.send_back(Msg::Response),
            task: None,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Request => {
                let request = CommentCreateInfoWrapper {
                    comment: self.request.clone(),
                };
                self.task = Some(self.comments.create(
                    self.props.slug.clone(),
                    request,
                    self.response.clone(),
                ));
            }
            Msg::Response(Ok(comment_info)) => {
                self.props.callback.emit(comment_info.comment);
                self.error = None;
                self.task = None;
                self.request = CommentCreateInfo::default();
            }
            Msg::Response(Err(err)) => {
                self.error = Some(err);
                self.task = None;
            }
            Msg::UpdateComment(body) => {
                self.request.body = body;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <>
                <ListErrors error=&self.error />
                <form class="card comment-form" onsubmit=|ev| { ev.prevent_default(); Msg::Request }>
                    <div class="card-block">
                        <textarea class="form-control"
                            placeholder="Write a comment..."
                            rows="3"
                            value=&self.request.body
                            oninput=|ev| Msg::UpdateComment(ev.value) >
                        </textarea>
                    </div>
                    <div class="card-footer">
                        {if let Some(image) = &self.props.current_user.image {
                            html! {
                                <img
                                    src={ image }
                                    class="comment-author-img"
                                    alt={ &self.props.current_user.username} />
                            }
                        } else {
                            html! { }
                        }}
                        <button
                            class="btn btn-sm btn-primary"
                            type="submit">
                            { "Post Comment" }
                        </button>
                    </div>
                </form>
            </>
        }
    }
}
