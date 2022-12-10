use std::fmt;

use dioxus::prelude::*;
use uuid::Uuid;


const STYLE: &str = include_str!("./style.css");

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Order {
    First,
    Middle,
    Last
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Order::First => write!(f, "message-first"),
            Order::Middle => write!(f, "message-middle"),
            Order::Last => write!(f, "message-last"),
        }
    }
}

#[derive(Props)]
pub struct Props<'a> {
    #[props(optional)]
    user_image: Option<Element<'a>>,
    #[props(optional)]
    loading: Option<bool>,
    #[props(optional)]
    with_text: Option<String>,
    #[props(optional)]
    remote: Option<bool>,
    #[props(optional)]
    remote_message: Option<bool>,
    #[props(optional)]
    with_prefix: Option<String>,
}

#[allow(non_snake_case)]
pub fn MessageReply<'a>(cx: Scope<'a,Props<'a>>) -> Element<'a> {
    let UUID = Uuid::new_v4().to_string();
    let scoped_styles = STYLE.replace("UUID", &UUID);

    let text = cx.props.with_text.clone().unwrap_or_default();
    let prefix = cx.props.with_prefix.clone().unwrap_or_default();

    let loading = cx.props.loading.unwrap_or_default();
    let remote = cx.props.remote.unwrap_or_default();
    let remote_message = cx.props.remote_message.unwrap_or_default();
    
    cx.render(rsx! (
        style { "{scoped_styles}" },
        div {
            class: {
                format_args!(
                    "message-reply-{} {} {}", 
                    &UUID,
                    if loading {
                        "loading"
                    } else { "" },
                    if remote {
                        "remote"
                    } else { "" },
                )
            },
            (cx.props.user_image.is_some() && remote_message).then(|| rsx! (
                &cx.props.user_image
            )),
            (cx.props.with_text.is_some()).then(|| rsx! (
                div {
                    class: "content",
                    (!prefix.is_empty()).then(|| rsx!(
                        p {
                            class: "prefix",
                            "{prefix}"
                        },
                    )),
                    p {
                        class: {
                            format_args!("text {}", if remote_message { "remote-text" } else { "" })
                        },
                        
                        "{text}"
                    }
                }
            )),
            (cx.props.user_image.is_some() && !remote_message).then(|| rsx! (
                &cx.props.user_image
            )),
        }
    ))
}