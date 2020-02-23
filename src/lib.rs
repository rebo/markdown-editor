#![feature(track_caller)]
use comp_state::*;
use comp_state_seed_extras::*;
use comrak::{markdown_to_html, ComrakOptions};
use seed::{prelude::*, *};

#[derive(Default)]
struct Model {}

enum Msg {
    NoOp,
    SubmitMarkdownHtml(String),
}

impl std::default::Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SubmitMarkdownHtml(html) => log!(html),
        Msg::NoOp => {}
    }
}

#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    markdown_editor(Msg::SubmitMarkdownHtml)
}

fn set_scroll(textarea: web_sys::HtmlTextAreaElement, preview: web_sys::HtmlElement) {
    let scroll_percentage = (textarea.scroll_top() as f64) / (textarea.scroll_height() as f64);
    let new_scroll_top = (preview.scroll_height() as f64) * scroll_percentage;
    preview.set_scroll_top(new_scroll_top as i32);
}

#[topo::nested]
fn markdown_editor(msg_handler: impl FnOnce(String) -> Msg + 'static + Clone) -> Node<Msg> {
    let source = use_state(|| String::new());
    let preview_el = use_state::<ElRef<web_sys::HtmlElement>, _>(ElRef::default);
    let textarea_el = use_state::<ElRef<web_sys::HtmlTextAreaElement>, _>(ElRef::default);

    let processed_md = markdown_to_html(&source.get(), &ComrakOptions::default());

    div![
        class!["flex flex-col"],
        div![
            class!["flex flex-row"],
            div![class!("w-1/2"), "Markdown:"],
            div![class!("w-1/2"), "Preview:"],
        ],
        div![
            class!["flex" "flex-row" "h-64"],
            textarea![
                el_ref(&textarea_el.get()),
                bind(At::Value, source),
                class!["font-mono p-2 h-full flex-none w-1/2 border-gray-200 border shadow-lg"],
                attrs![At::Type => "textbox"],
                textarea_el.input_ev(Ev::KeyUp, move |el, _| {
                    if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
                        set_scroll(textarea, preview);
                    }
                }),
                textarea_el.input_ev(Ev::Scroll, move |el, _| {
                    if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
                        set_scroll(textarea, preview);
                    }
                })
            ],
            div![
                class!["md-preview"],
                el_ref(&preview_el.get()),
                class!["overflow-auto p-2 pl-4 h-full flex-none w-1/2 border-gray-200 bg-indigo-100 border shadow-lg"],
                raw!(&processed_md)
            ]
        ],
        div![
            class!["flex justify-end pt-2"],
            button![
                class!["bg-green-400 p-4 m-2"],
                "Submit",
                mouse_ev(Ev::Click, move |_| msg_handler(processed_md))
            ]
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
