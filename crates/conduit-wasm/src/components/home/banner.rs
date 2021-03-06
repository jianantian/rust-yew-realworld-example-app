use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::agent::is_authenticated;

pub struct Banner {}

pub enum Msg {}

impl Component for Banner {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Banner {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        if is_authenticated() {
            html! {}
        } else {
            html! {
                <div class="banner">
                    <div class="container">
                        <h1 class="logo-font">
                            { "conduit" }
                        </h1>
                        <p>{ "A place to share your knowledge." }</p>
                    </div>
                </div>
            }
        }
    }
}
