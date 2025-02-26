use gtk::prelude::*;
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use relm4::factory::FactoryVecDeque;
use relm4::prelude::{AsyncComponentParts, AsyncComponentSender, SimpleAsyncComponent};
use relm4::{gtk, RelmApp, RelmWidgetExt};

use std::env;
mod modules;
use modules::components::MessageBubble;

struct AppModel {
    messages: FactoryVecDeque<MessageBubble>,
    scrolled_window: Option<gtk::ScrolledWindow>,
    text_entry: Option<gtk::Entry>,
    chat_messages: Vec<chat_completion::ChatCompletionMessage>,
}

#[derive(Debug)]
enum AppMsg {
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
						connect_activate => AppMsg::SendPrompt,
                        // connect_changed[sender] => move |entry| {
                        //     sender.input(AppMsg::UpdateInput(entry.text().to_string()));
                        // }
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
            text_entry: None,
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
		model.text_entry = Some(widgets.entry_field.clone());

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        let scroll_to_bottom = || {
            let scrolled_window = self
                .scrolled_window
                .clone()
                .expect("ScrolledWindow is not initialized!");

			let immediate_scroll = scrolled_window.clone();
            glib::idle_add_local_once(move || {
                let adj = immediate_scroll.vadjustment();
                adj.set_value(adj.upper() - adj.page_size());
            });
			
			// I don't know why this is necessary, but without it, the scroll to bottom cannot
			// work when sending a message.
            glib::timeout_add_local_once(std::time::Duration::from_millis(100), move || {
                let adj = scrolled_window.vadjustment();
                adj.set_value(adj.upper() - adj.page_size());
            });
        };

        match msg {
            AppMsg::SendPrompt => {
				let text_entry = self.text_entry.clone().expect("TextEntry is not initialized!");
                let text = text_entry.text().to_string();
                if text.is_empty() {
                    return;
                }
				text_entry.set_text("");

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

                scroll_to_bottom();

                // Clear input
                // sender.input(AppMsg::UpdateInput(String::new()));

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
            }
            AppMsg::ReceiveResponse(result) => {
                let text = match result {
                    Ok(Some(response)) => response,
                    Ok(None) => "No response".to_string(),
                    Err(e) => format!("ERROR: {}", e.to_string()),
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

                scroll_to_bottom();
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
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => return Err("env OPENAI_API_KEY not set".into()),
    };
    let client = OpenAIClient::builder()
        .with_endpoint("https://api.siliconflow.cn/v1")
        .with_api_key(api_key.to_string())
        .build()?;

    let req = ChatCompletionRequest::new(
        "Qwen/Qwen2.5-7B-Instruct".to_string(),
        messages.clone(),
    );

    let result = client.chat_completion(req).await?;
    // println!("Received response from LLM: {:?}", result.choices[0].message.content.clone());

    Ok(result.choices[0].message.content.clone())
}
