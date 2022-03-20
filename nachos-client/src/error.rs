use yew::{Context, Html};

#[derive(yew::Properties, PartialEq)]
pub struct Error {
    pub(crate) description: String,
}

impl yew::Component for Error {
    type Message = ();
    type Properties = Self;

    fn create(ctx: &Context<Self>) -> Self {
        Error {
            description: ctx.props().description.clone(),
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        yew::html! {
            <>
                <h1>{ "Error" }</h1>
                <h2>{ &self.description }</h2>
            </>
        }
    }
}
