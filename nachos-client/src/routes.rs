use yew_router::components::Link;
use crate::session::Session as NachosSession;

#[derive(Clone, PartialEq, yew_router::Routable)]
pub enum Routes {
    #[at("/")]
    Home,
    #[at("/session/:id")]
    Session {
        id: String
    },
}

impl Routes {
    pub fn switch(routes: &Routes) -> yew::Html {
        use Routes::*;

        match routes {
            Home => yew::html! { <><button><Link<Self> to={Self::Session { id: "1234".to_string() }}>{ "Connect" }</Link<Self>></button></> },
            Session {..} => yew::html! { <NachosSession /> },
        }
    }
}
