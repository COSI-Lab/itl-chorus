use gloo::file::callbacks::FileReader;
use gloo::file::File;
use reqwest::{
    multipart::{Form, Part},
    Client,
};
use std::collections::HashMap;
use std::borrow::Cow;
use web_sys::{Event, FileList, HtmlInputElement};
use yew::html::TargetCast;
use yew::{html, Component, Context, Html};

async fn upload(file_name: String, data: Vec<u8>) -> Result<reqwest::Response, reqwest::Error> {
    let part = Part::bytes(Cow::from(data)).file_name(file_name.clone());
    let form = Form::new().part("upload", part);

    reqwest::Client::new()
        .post("http://localhost:8081/midi")
        .multipart(form)
        .send()
        .await
}

pub enum Msg {
    Uploaded(String, bool),
    Loaded(String, String, Vec<u8>),
    Files(Vec<File>),
}

pub struct UploadMidi {
    readers: HashMap<String, FileReader>,
}

impl Component for UploadMidi {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Uploaded(_, _) => {
                true
            }
            Msg::Loaded(file_name, _file_type, data) => {
                self.readers.remove(&file_name);
                ctx.link().send_future(async move {
                    let res = upload(file_name.clone(), data).await;

                    match res {
                        Ok(_) => Msg::Uploaded(file_name.clone(), true),
                        Err(err) => {
                            log::error!("{}", err);
                            Msg::Uploaded(file_name.clone(), false)
                        }
                    }
                });
                false
            }
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let file_type = file.raw_mime_type();

                    let task = {
                        let link = ctx.link().clone();
                        let file_name = file_name.clone();

                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_name,
                                file_type,
                                res.expect("failed to read file"),
                            ))
                        })
                    };

                    self.readers.insert(file_name, task);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="wrapper">
                <input
                    id="file-upload"
                    type="file"
                    accept="audio/midi"
                    multiple={true}
                    onchange={ctx.link().callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Self::upload_files(input.files())
                    })}
                />
                <p> {{ self.readers.len() }} </p>
            </div>
        }
    }
}

impl UploadMidi {
    fn upload_files(files: Option<FileList>) -> Msg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }

        Msg::Files(result)
    }
}
