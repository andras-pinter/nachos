mod props;
mod state;

use crate::error::Error;
use crate::protos::SshClient;
use common::{Api, Connection};
use gloo_net::http::Request;
use yew::{Context, Html};

pub struct Session {
    id: String,
    error: Option<String>,
}

impl yew::Component for Session {
    type Message = state::SessionState;
    type Properties = props::SessionProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let session_id = ctx.props().id.clone();
        ctx.link().send_message(state::SessionState::Init);
        ctx.link().send_future(async {
            Request::get(&Api::Session.call(session_id))
                .send()
                .await?
                .json::<Connection>()
                .await
        });
        Session {
            id: ctx.props().id.clone(),
            error: None,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        use state::SessionState::*;

        match msg {
            Ready => true,
            Error(err) => {
                self.error = Some(err);
                true
            }
            _ => false,
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        if let Some(err) = self.error.clone() {
            yew::html! { <Error description={ err } /> }
        } else {
            yew::html! {
                <SshClient
                    session_id={ self.id.clone() }
                />
            }
        }
    }
}
