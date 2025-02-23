use glib;
use gtk::prelude::*;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use relm4::prelude::{AsyncComponentParts, SimpleAsyncComponent};
use relm4::{gtk, AsyncComponentSender, RelmApp, RelmWidgetExt};
use std::env;
use gtk4::WrapMode;

struct AppModel {
    text: String,
}

#[derive(Debug)]
enum AppMsg {
    SendPrompt(String),
}

#[relm4::component(async)]
impl SimpleAsyncComponent for AppModel {
    type Init = String;

    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Simple app"),
            set_default_width: 400,
            set_default_height: 300,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 4,
                set_margin_all: 8,

                gtk::ScrolledWindow {
                    #[name = "text_view"]
                    gtk::TextView {
                        set_editable: false,
                        set_hexpand: true,
                        set_vexpand: true,
						set_wrap_mode: WrapMode::Word,
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 2,
                    set_margin_all: 0,

                    #[name = "entry_field"]
                    gtk::Entry {
                        set_placeholder_text: Some("Enter text"),
                        set_hexpand: true,
                    },
                    #[name = "submit_button"]
                    gtk::Button {
                        set_label: "Submit",
                        connect_clicked => AppMsg::SendPrompt("hello".to_string()),
                    },

                }
            }
        }
    }

    // Initialize the UI.
    async fn init(
        text: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = AppModel { text };

        // Insert the macro code generation here
        let widgets = view_output!();
        let entry_field = widgets.entry_field.clone();
        let text_view = widgets.text_view.clone();
        widgets.submit_button.connect_clicked(move |_| {
            let prompt = entry_field.text().to_string();

            let buffer = text_view.buffer().clone();
            let mut end_iter = buffer.end_iter();
            buffer.insert(&mut end_iter, &format!("User input: {}\n", prompt));
			let text_view = text_view.clone();
            // println!("User input: {}", prompt);
            glib::spawn_future_local(async move {
                let response = get_llm_response(&prompt).await;
				let buffer = text_view.buffer();
            	let mut end_iter = buffer.end_iter();
                match response {
                    Ok(Some(response_text)) => {
                        let msg = &format!("LLM response: {}\n", response_text);
                        buffer.insert(&mut end_iter, msg);
                        // println!("LLM response: {}", msg);
                    }
                    Ok(None) => {
						let msg = &String::from("No response received");
						buffer.insert(&mut end_iter, msg);
                        println!("{}", msg);
                    }
                    Err(e) => {
						let msg = &format!("Error: {}", e);
						buffer.insert(&mut end_iter, msg);
                        println!("{}", msg);
                        // Handle the case where no response is received	
                    }
                }
				let end_mark = buffer.create_mark(None, &buffer.end_iter(), false);
                text_view.scroll_to_mark(&end_mark, 0.0, true, 0.0, 0.0);
            });
        });
        //widgets.text_view

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
        match msg {
            AppMsg::SendPrompt(text) => {
                // println!("Received prompt: {}", text);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");
    app.run_async::<AppModel>("".to_string());
}

async fn get_llm_response(
    prompt: &String,
) -> Result<Option<String>, std::boxed::Box<dyn std::error::Error>> {
    // println!("Sending prompt to LLM: {}", prompt);
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let client = OpenAIClient::builder()
        .with_endpoint("https://api.siliconflow.cn/v1")
        .with_api_key(api_key)
        .build()?;

    let req = ChatCompletionRequest::new(
        "Qwen/Qwen2-7B-Instruct".to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(prompt.to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
    );

    let result = client.chat_completion(req).await?;

    Ok(result.choices[0].message.content.clone())
}

// use gtk4::prelude::*;
// use relm4::prelude::*;
// use gtk4::glib;
// use std::time::Duration;
// use chrono::Local;
// use tokio::sync::mpsc::{self, Sender};

// #[derive(Debug)]
// struct AppModel {
//     tx: Sender<String>,
// }

// #[derive(Debug)]
// enum AppMsg {
//     Append(String),
// }

// #[relm4::component(async)]
// impl SimpleAsyncComponent for AppModel {
//     type Init = ();
//     type Input = AppMsg;
//     type Output = ();

//     view! {
//         gtk4::ApplicationWindow {
//             set_title: Some("Automatic Scrolling"),
//             set_default_width: 400,
//             set_default_height: 300,

//             gtk4::ScrolledWindow {
//                 set_hscrollbar_policy: gtk4::PolicyType::Never,
//                 #[name = "text_view"]
//                 gtk4::TextView {
//                     set_editable: false,
//                     set_cursor_visible: false,
//                     set_wrap_mode: gtk4::WrapMode::Word,
//                 }
//             }
//         }
//     }

//     async fn init(
//         _init: Self::Init,
//         root: Self::Root,
//         sender: AsyncComponentSender<Self>,
//     ) -> AsyncComponentParts<Self> {
//         let widgets = view_output!();
//         let text_view = &widgets.text_view;
//         let (tx, rx) = mpsc::channel(100);
//         let model = AppModel { tx };
//         setup_scroll(text_view, rx);
//         start_background_task(sender.clone());

//         AsyncComponentParts { model, widgets }
//     }

//     async fn update(&mut self, msg: Self::Input, _sender: AsyncComponentSender<Self>) {
//         match msg {
//             AppMsg::Append(text) => {
//                 self.tx.send(text).await.unwrap();
//             }
//         }
//     }
// }

// fn setup_scroll(text_view: &gtk4::TextView, mut receiver: mpsc::Receiver<String>) {
//     let buffer = text_view.buffer();
//     let end_mark = buffer.create_mark(Some("end"), &buffer.end_iter(), false);
//     let text_view = text_view.clone();

//     glib::spawn_future_local(async move {
//         while let Some(text) = receiver.recv().await {
//             let buffer = text_view.buffer();
//             let mut iter = buffer.end_iter();
//             buffer.insert(&mut iter, &format!("{}\n", text));
//             text_view.scroll_mark_onscreen(&end_mark);
//         }
//     });
// }

// fn start_background_task(sender: AsyncComponentSender<AppModel>) {
//     glib::spawn_future_local(async move {
//         loop {
//             let current_time = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
//             sender.input(AppMsg::Append(current_time));
//             glib::timeout_future(Duration::from_millis(100)).await;
//         }
//     });
// }

// fn main() {
//     let app = RelmApp::new("com.example.TextScroll");
//     app.run_async::<AppModel>(());
// }

// use std::time::Duration;

// use gtk::prelude::*;
// use relm4::{
//     component::{AsyncComponent, AsyncComponentParts, AsyncComponentSender},
//     gtk,
//     loading_widgets::LoadingWidgets,
//     view, RelmApp, RelmWidgetExt,
// };

// struct App {
//     counter: u8,
// }

// #[derive(Debug)]
// enum Msg {
//     Increment,
//     Decrement,
// }

// #[relm4::component(async)]
// impl AsyncComponent for App {
//     type Init = u8;
//     type Input = Msg;
//     type Output = ();
//     type CommandOutput = ();

//     view! {
//         gtk::Window {
//             gtk::Box {
//                 set_orientation: gtk::Orientation::Vertical,
//                 set_spacing: 5,
//                 set_margin_all: 5,

//                 gtk::Button {
//                     set_label: "Increment",
//                     connect_clicked => Msg::Increment,
//                 },

//                 gtk::Button {
//                     set_label: "Decrement",
//                     connect_clicked => Msg::Decrement,
//                 },

//                 gtk::Label {
//                     #[watch]
//                     set_label: &format!("Counter: {}", model.counter),
//                     set_margin_all: 5,
//                 }
//             }
//         }
//     }

//     fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
//         view! {
//             #[local]
//             root {
//                 set_title: Some("Simple app"),
//                 set_default_size: (300, 100),

//                 // This will be removed automatically by
//                 // LoadingWidgets when the full view has loaded
//                 #[name(spinner)]
//                 gtk::Spinner {
//                     start: (),
//                     set_halign: gtk::Align::Center,
//                 }
//             }
//         }
//         Some(LoadingWidgets::new(root, spinner))
//     }

//     async fn init(
//         counter: Self::Init,
//         root: Self::Root,
//         sender: AsyncComponentSender<Self>,
//     ) -> AsyncComponentParts<Self> {
//         tokio::time::sleep(Duration::from_secs(1)).await;

//         let model = App { counter };

//         // Insert the code generation of the view! macro here
//         let widgets = view_output!();

//         AsyncComponentParts { model, widgets }
//     }

//     async fn update(
//         &mut self,
//         msg: Self::Input,
//         _sender: AsyncComponentSender<Self>,
//         _root: &Self::Root,
//     ) {
//         tokio::time::sleep(Duration::from_secs(1)).await;
//         match msg {
//             Msg::Increment => {
//                 self.counter = self.counter.wrapping_add(1);
//             }
//             Msg::Decrement => {
//                 self.counter = self.counter.wrapping_sub(1);
//             }
//         }
//     }
// }

// fn main() {
//     let app = RelmApp::new("relm4.example.simple_async");
//     app.run_async::<App>(0);
// }
