mod bindings;
mod app;
mod session;
mod routes;

fn main() {
    yew::start_app::<app::Nachos>();
}
