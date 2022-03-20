mod props;
mod terminal;

use futures::StreamExt;
use gloo_net::websocket::Message;
use gloo_net::websocket::futures::WebSocket;
use wasm_bindgen_futures::spawn_local;
use crate::error::Error;
use common::Api;
pub use props::SshClientProperties;
use yew::{Context, Html};

const TERMINAL_ID: &str = "nachosTerminal";
const TERMINAL_STYLE: &str =
    "overflow:hidden;width:100%;height:100%;font-size:10px;line-height:17px;margin:0;";
const APP_COMPONENT_CSS: &str = "xterm.css";
const APP_COMPONENT_JS: &str = "xterm.js";

pub enum State {
    Ready,
    Error(&'static str),
}

pub struct SshClient {
    session_id: String,
    error: Option<&'static str>,
}

impl yew::Component for SshClient {
    type Message = State;
    type Properties = SshClientProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self::init();

        SshClient {
            session_id: ctx.props().session_id.clone(),
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            State::Ready => {
                let term = terminal::Terminal::new();
                term.open(TERMINAL_ID);
                let ws = Self::tunnel(&self.session_id).and_then(|url| {
                    WebSocket::open(&url).map_err(|_| "Failed to open WebSocket tunnel")
                });

                match ws {
                    Ok(mut ws) => {
                        let (_, mut read) = ws.split();
                        spawn_local(async move {
                            while let Some(msg) = read.next().await {
                                match msg {
                                    Ok(Message::Bytes(data)) => term.write(data),
                                    _ => ()
                                }
                            }
                        });
                    },
                    Err(err) => {
                        gloo_console::error!(err);
                        ctx.link().send_message(State::Error(err));
                    }
                }
                true
            }
            State::Error(err) => {
                self.error = Some(err);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.error {
            None => {
                let onload = ctx.link().callback(|_| State::Ready);

                yew::html! {
                    <>
                        <div id={ TERMINAL_ID } style={ TERMINAL_STYLE }></div>
                        <script src={ APP_COMPONENT_JS } {onload} />
                    </>
                }
            }
            Some(err) => yew::html! { <Error description={ err.to_string() } /> },
        }
    }
}

impl SshClient {
    fn init() {
        gloo_utils::body()
            .style()
            .set_css_text("margin:0;padding:0;background-color:#000000;");

        if let Ok(link) = Self::create_style_link_tag() {
            if gloo_utils::head().append_child(&link).is_err() {
                gloo_console::error!("Failed to append <link> element")
            }
        } else {
            gloo_console::error!("Failed to create <link> element")
        }
    }

    fn create_style_link_tag() -> Result<web_sys::Element, wasm_bindgen::JsValue> {
        let link = gloo_utils::document().create_element("link")?;
        link.set_attribute("rel", "stylesheet")?;
        link.set_attribute("href", APP_COMPONENT_CSS)?;

        Ok(link)
    }

    fn tunnel(session_id: &str) -> Result<String, &'static str> {
        let base = gloo_utils::document()
            .base_uri()
            .map_err(|_| "Failed to get base URI")?
            .ok_or("Failed to get base URI")?
            .trim_end_matches("/")
            .to_string();
        let mut url = option_env!("NACHOS_WS_SERVER")
            .map(ToString::to_string)
            .unwrap_or(base);
        url.push_str(&Api::tunnel(session_id));

        Ok(url.replace("http", "ws"))
    }
}
