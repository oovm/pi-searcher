use dioxus::{events::FormEvent, prelude::*};

use crate::hook::use_searcher;

pub fn Editor(cx: Scope) -> Element {
    let place_holder = r#"1415926"#;
    let text = use_state(&cx, || place_holder.to_string());
    let pi = use_searcher(&cx);
    let digit = pi.search(text);
    cx.render(rsx!(
        div {
            class: "flex flex-column justify-center",
            div {
                class: "form-control w-full max-w-sm",
                img {
                    src: "https://the-hollywood-gossip-res.cloudinary.com/iu/s--O281NiSS--/t_xlarge_l/cs_srgb,f_auto,fl_strip_profile.lossy,q_auto:420/v1489499750/attachment/pi-day-picture.png"
                }
                label {
                    class: "label",
                    span {
                        class: "label-text",
                        "What is your luck number?"
                    }
                    span {
                        class: "label-text-alt",
                        "Find Luck Pi!"
                    }
                }
                input {
                    class: "input input-bordered w-full max-w-sm",
                    placeholder: "Type here",
                    r#type: "text",
                    placeholder: "{place_holder}",
                    oninput: move |e| text.set(e.value.to_owned()),
                    value: "{text}",
                }
                div {
                    class: "flex-1 ml-2 mr-2 mt-2",
                    digit
                }
                a {
                    class: "mt-2",
                    href: "https://github.com/oovm/pi-searcher/issues",
                    target: "_blank",
                    button {
                        class: "py-2 px-4 mr-2 mb-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700",
                        r#type: "button",
                        "Report bug on github"
                    }
                }
            }
        }
    ))
}
