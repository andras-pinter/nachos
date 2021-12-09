use yew::{Context, Html, html};
use crate::bindings::NachosClient;

pub struct Session {
    client: NachosClient,
}

impl yew::Component for Session {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Session {
            client: NachosClient::new("ws://127.0.0.1:8000/tunnel"),
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        self.client.connect();
        html! {}
    }
}
