use comp_state::*;
use comp_state_seed_extras::*;
use seed::{prelude::*, *};
use web_sys::{HtmlElement, HtmlTextAreaElement};

pub fn markdown_editor_state<Ms, F>(md_state: StateAccess<String>) -> Node<Ms>
where
    F: FnOnce(String) -> Ms + 'static + Clone,
    Ms: Default + 'static,
{
    let source = use_state(|| String::new());
    let preview_el = use_state(ElRef::<HtmlElement>::default);
    let textarea_el = use_state(ElRef::<HtmlTextAreaElement>::default);

    div![
        class!["flex flex-col"],
        div![
            class!["flex flex-row"],
            div![class!["w-1/2"], "Markdown:"],
            div![class!["w-1/2"], "Preview:"],
        ],
        div![
            class!["flex flex-row h-64"],
            textarea![
                el_ref(&textarea_el.get()),
                bind(At::Value, source),
                class!["font-mono p-2 h-full flex-none w-1/2 border-gray-200 border shadow-lg"],
                attrs![At::Type => "textbox"],
                crate::scroll_event_handler(Ev::KeyUp ,textarea_el, preview_el),
                crate::scroll_event_handler(Ev::Scroll, textarea_el, preview_el),
            ],
            div![
                el_ref(&preview_el.get()),
                class!["markdown-body"],
                class!["overflow-auto p-2 pl-4 h-full flex-none w-1/2 border-gray-200 bg-indigo-100 border shadow-lg"],
                md!(&source.get())
            ]
        ],
        div![
            class!["flex justify-end pt-2"],
            button![
                class!["bg-green-400 p-4 m-2"],
                "Submit (See console log)",
                mouse_ev(Ev::Click, move |_| {
                    let markdown_element = preview_el.get().get().expect("markdown-body doesn't exist");
                    md_state.set(markdown_element.inner_html());
                    Ms::default()
                })
            ]
        ]
    ]
}
