use crate::routes::Routes;
use yew::{Context, Html};
use yew_router::{BrowserRouter, Switch};

pub struct Nachos;

impl yew::Component for Nachos {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        yew::html! {
            <BrowserRouter>
                <Switch<Routes> render={ Switch::render(Routes::switch) } />
            </BrowserRouter>
        }
    }
}
