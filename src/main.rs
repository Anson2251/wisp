use gtk::prelude::*;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use relm4::factory::{FactoryComponent, FactorySender, FactoryVecDeque};
use relm4::prelude::{
    AsyncComponentParts, AsyncComponentSender, DynamicIndex, SimpleAsyncComponent,
};
use relm4::{gtk, RelmApp, RelmWidgetExt};

use std::env;

struct MessageRow {
    text: String,
    is_user: bool,
}

#[relm4::factory]
impl FactoryComponent for MessageRow {
    type Init = (String, bool);
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            set_halign: if self.is_user { gtk::Align::End } else { gtk::Align::Start },

            gtk::Label {
                set_label: &self.text,
                set_wrap: true,
                set_max_width_chars: 40,
                set_css_classes: &["message-bubble", (if self.is_user { "user-bubble" } else {"llm-bubble"})],
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        MessageRow {
            text: init.0,
            is_user: init.1,
        }
    }
}

struct AppModel {
    messages: FactoryVecDeque<MessageRow>,
    scrolled_window: Option<gtk::ScrolledWindow>,
    input_buffer: String,
    chat_messages: Vec<chat_completion::ChatCompletionMessage>,
}

#[derive(Debug)]
enum AppMsg {
    UpdateInput(String),
    SendPrompt,
    ReceiveResponse(Result<Option<String>, String>),
}

#[relm4::component(async)]
impl SimpleAsyncComponent for AppModel {
    type Init = String;
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Relm4 Chat"),
            set_default_width: 400,
            set_default_height: 300,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 0,
                set_margin_all: 0,

                #[name = "scrolled_window"]
                gtk::ScrolledWindow {
                    set_vexpand: true,

                    #[local_ref]
                    messages_container -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 2,
                    set_margin_all: 4,

                    #[name = "entry_field"]
                    gtk::Entry {
                        set_placeholder_text: Some("Enter message..."),
                        set_hexpand: true,
                        connect_changed[sender] => move |entry| {
                            sender.input(AppMsg::UpdateInput(entry.text().to_string()));
                        }
                    },

                    gtk::Button {
                        set_label: "Send",
                        connect_clicked => AppMsg::SendPrompt,
                    },
                }
            }
        }
    }

    async fn init(
        init_text: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let messages = FactoryVecDeque::builder()
            .launch(gtk::Box::new(gtk::Orientation::Vertical, 8))
            .detach();

        let mut model = AppModel {
            messages,
            scrolled_window: None,
            input_buffer: String::new(),
            chat_messages: vec![chat_completion::ChatCompletionMessage {
                role: chat_completion::MessageRole::system,
                content: chat_completion::Content::Text(init_text),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }],
        };

        let messages_container = model.messages.widget();
        let widgets = view_output!();

        model.scrolled_window = Some(widgets.scrolled_window.clone());

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            AppMsg::UpdateInput(text) => {
                self.input_buffer = text;
            }
            AppMsg::SendPrompt => {
                let text = self.input_buffer.trim().to_string();
                if text.is_empty() {
                    return;
                }

                // Add user message
                self.messages.guard().push_back((text.clone(), true));
                self.chat_messages
                    .push(chat_completion::ChatCompletionMessage {
                        role: chat_completion::MessageRole::user,
                        content: chat_completion::Content::Text(text.clone()),
                        name: None,
                        tool_calls: None,
                        tool_call_id: None,
                    });

                // Clear input
                self.input_buffer.clear();
                sender.input(AppMsg::UpdateInput(String::new()));

                // Request AI response
                let history = self.chat_messages.clone();
                let sender_clone = sender.clone();
                tokio::spawn(async move {
                    match get_llm_response(&history).await {
                        Ok(Some(response)) => {
                            sender_clone.input(AppMsg::ReceiveResponse(Ok(Some(response))));
                        }
                        Ok(None) => {
                            sender_clone
                                .input(AppMsg::ReceiveResponse(Err("No response".to_string())));
                        }
                        Err(e) => {
                            sender_clone.input(AppMsg::ReceiveResponse(Err(e.to_string())));
                        }
                    }
                });

                // Scroll to bottom
                let scrolled_window = self
                    .scrolled_window
                    .as_ref()
                    .expect("ScrolledWindow is not initialized!");
                let adj = scrolled_window.vadjustment();
                adj.set_value(adj.upper() - adj.page_size());
            }
            AppMsg::ReceiveResponse(result) => {
                let text = match result {
                    Ok(Some(response)) => response,
                    Ok(None) => "No response".to_string(),
                    Err(e) => e.to_string(),
                };

                self.messages.guard().push_back((text.clone(), false));

                self.chat_messages
                    .push(chat_completion::ChatCompletionMessage {
                        role: chat_completion::MessageRole::assistant,
                        content: chat_completion::Content::Text(text),
                        name: None,
                        tool_calls: None,
                        tool_call_id: None,
                    });

                // Scroll to bottom
                let scrolled_window = self
                    .scrolled_window
                    .as_ref()
                    .expect("ScrolledWindow is not initialized!");
                let adj = scrolled_window.vadjustment();
                adj.set_value(adj.upper() - adj.page_size());
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");

    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("style.css"));
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    app.run_async::<AppModel>("".to_string());
}

async fn get_llm_response(
    messages: &Vec<chat_completion::ChatCompletionMessage>,
) -> Result<Option<String>, std::boxed::Box<dyn std::error::Error>> {
    // println!("Sending prompt to LLM: {}", prompt);
    let api_key =
        env::var("OPENAI_API_KEY").expect("Load Open AI API key from env OPENAI_API_KEY failed");
    let client = OpenAIClient::builder()
        .with_endpoint("https://api.siliconflow.cn/v1")
        .with_api_key(api_key.to_string())
        .build()?;

    let req = ChatCompletionRequest::new(
        "meta-llama/Meta-Llama-3.1-8B-Instruct".to_string(),
        messages.clone(),
    );

    let result = client.chat_completion(req).await?;
    // println!("Received response from LLM: {:?}", result.choices[0].message.content.clone());

    Ok(result.choices[0].message.content.clone())
}
