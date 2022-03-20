mod app;
mod error;
mod protos;
mod routes;
mod session;

fn main() {
    yew::start_app::<app::Nachos>();
}
