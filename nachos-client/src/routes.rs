use crate::session::Session;

#[derive(Clone, PartialEq, yew_router::Routable)]
pub enum Routes {
    #[at("/session/:id")]
    Session { id: String },
    #[cfg(feature = "testConnections")]
    #[at("/")]
    Home,
}

impl Routes {
    pub fn switch(route: &Routes) -> yew::Html {
        match route.clone() {
            Routes::Session { id } => yew::html! { <Session id={ id } /> },
            #[cfg(feature = "testConnections")]
            Routes::Home => yew::html! {
                <button><yew_router::prelude::Link<Self> to={Self::Session { id: "testSsh".to_string() }}>{ "SSH" }</yew_router::prelude::Link<Self>></button>
            },
        }
    }
}
