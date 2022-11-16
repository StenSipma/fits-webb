use yew::prelude::*;
use web_sys::{HtmlInputElement, console};
use js_sys;
use gloo_file::File;
use gloo_file::callbacks::FileReader;

mod fits_element;
use crate::fits_element::FitsElement;

enum Msg {
    Read(File),
    Loaded(Vec<u8>),
    Nothing,
}

struct Model {
    reader: Option<FileReader>,
    file: Option<File>,
    data: Option<Vec<u8>>
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    /// Initialize the component
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            reader: None,
            file: None,
            data: None,
        }
    }

    /// Function that is called when it gets a message?
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        match msg {
            Msg::Read(file) => {
                self.file = Some(file.clone());
                // Send data to be read into a string
                let reader = gloo_file::callbacks::read_as_bytes(&file, move |res| {
                    link.send_message(Msg::Loaded(res.unwrap()));
                });
                self.reader = Some(reader);
            },
            Msg::Nothing => {
                self.file = None;
                self.data = None;
            },
            Msg::Loaded(res) => {
                console::log_1(&format!("Loaded a file ").into());
                self.data = Some(res);
                self.reader = None;
            },
        };
        true
    }

    /// Function that is called when viewing the page, similar to something like 'draw'
    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let file = self.file.as_ref();

        // Callback to read the file from the file browser input
        let onchange= link.callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        if let Some(file_list) = input.files() {
                            let files: Vec<File> = js_sys::try_iter(&file_list)
                                .unwrap()
                                .unwrap()
                                .map(|f| web_sys::File::from(f.unwrap()))
                                .map(File::from)
                                .collect();
                            if let Some(file) = files.first() {
                                Msg::Read(file.clone())
                            } else {
                                Msg::Nothing
                            }
                        } else {
                            Msg::Nothing
                        }
                    });
        let data = self.data.as_ref();
        html! {
            <>
                <div class="row">
                    <label class="file_input">{"Input a file:"}</label>
                    <input type="file" {onchange} />
                </div>
                <div class="row">
                if file.is_some() && data.is_some() {
                    <FitsElement file={file.unwrap().clone()} data={data.unwrap().clone()}/>
                } else {
                    <div>
                        <h3>{"Select a file..."}</h3>
                    </div>
                }
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
