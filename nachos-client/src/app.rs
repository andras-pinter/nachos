use yew::{Context, Html};
use yew_router::{BrowserRouter, Switch};
use crate::routes::Routes;

pub struct Nachos;

impl yew::Component for Nachos {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        yew::html! {
            <>
                <h1>{ "Nachos" }</h1>
                <BrowserRouter>
                    <Switch<Routes> render={ Switch::render(Routes::switch) } />
                </BrowserRouter>
            </>
        }
    }
}
