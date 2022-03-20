use rand::{Rng, Fill, thread_rng};
use yew::{Context, Html};

pub fn payload(l: usize) -> Vec<u8> {
    let mut payload = Vec::<u8>::with_capacity(l);
    payload.resize(l, 0);
    for i in payload.iter_mut() {
        *i = thread_rng().gen_range(0..10);
    }

    payload
}

struct Payload {

}

impl yew::Component for Payload {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        Html::default()
    }
}
