#![feature(track_caller)]
use comp_state::*;
use comp_state_seed_extras::*;
use seed::{prelude::*, *};
use web_sys::{HtmlElement, HtmlTextAreaElement};
mod alt_md;
#[derive(Default)]
struct Model {}

enum Msg {
    NoOp,
    SubmitMarkdownHtml(String),
}

impl Default for Msg {
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

fn markdown_editor<Ms, F>(on_submit: F) -> Node<Ms>
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
                scroll_event_handler(Ev::KeyUp ,textarea_el, preview_el),
                scroll_event_handler(Ev::Scroll, textarea_el, preview_el),
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
                    on_submit(markdown_element.inner_html())
                })
            ]
        ]
    ]
}

fn scroll_event_handler<Ms>(
    event: Ev,
    textarea_el: StateAccess<ElRef<HtmlTextAreaElement>>,
    preview_el: StateAccess<ElRef<HtmlElement>>,
) -> EventHandler<Ms>
where
    Ms: 'static + Default,
{
    textarea_el.input_ev(event, move |el, _| {
        if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
            let textarea_scroll_percentage = {
                let textarea_max_scroll_top = textarea.scroll_height() - textarea.client_height();
                if textarea_max_scroll_top == 0 {
                    0.
                } else {
                    f64::from(textarea.scroll_top()) / f64::from(textarea_max_scroll_top)
                }
            };
            let new_preview_scroll_top = {
                let preview_max_scroll_top = preview.scroll_height() - preview.client_height();
                f64::from(preview_max_scroll_top) * textarea_scroll_percentage
            };
            preview.set_scroll_top(new_preview_scroll_top as i32);
        }
    })
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
