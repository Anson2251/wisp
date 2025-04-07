#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Utc;
use gtk::prelude::*;
use modules::database::Conversation;
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
    db: modules::database::Database,
    current_conversation: Option<i64>,
    conversations: Vec<modules::database::Conversation>,
}

#[derive(Debug)]
enum AppMsg {
    SendPrompt,
    ReceiveResponse(Result<Option<String>, String>),
    ConversationSelected(i64),
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
            set_orientation: gtk::Orientation::Horizontal,

            #[name = "conversations_sidebar"]
            gtk::ScrolledWindow {
                set_size_request: (200, -1),
                set_vexpand: true,

				#[name = "conv_list"]
                gtk::ListBox {
                    set_selection_mode: gtk::SelectionMode::Single,
                    set_vexpand: true,
                    connect_row_selected[sender] => move |_, row| {
							if let Some(row) = row {
								let conv_id = row.property::<i64>("conversation-id");
								sender.input(AppMsg::ConversationSelected(conv_id));
							}
                        }
                    }
                }
            },

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

        let db = match modules::database::Database::new() {
            Ok(db) => db,
            Err(e) => {
                eprintln!("Failed to initialize database: {}", e);
                std::process::exit(1);
            }
        };

        let conversations = match db.list_conversations() {
            Ok(convs) => convs,
            Err(e) => {
                eprintln!("Failed to load conversations: {}", e);
                Vec::new()
            }
        };

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
            db,
            current_conversation: None,
            conversations,
        };

        let messages_container = model.messages.widget();
        let widgets = view_output!();

        // Populate conversations list
        for conv in &model.conversations {
            let row = gtk::ListBoxRow::new();
            let label = gtk::Label::new(Some(&conv.title));
            label.set_xalign(0.0);
            row.set_child(Some(&label));
            row.set_property("conversation-id", conv.id);
            widgets.conv_list.append(&row);
        }

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
                let text_entry = self
                    .text_entry
                    .clone()
                    .expect("TextEntry is not initialized!");
                let text = text_entry.text().to_string();
                if text.is_empty() {
                    return;
                }
                text_entry.set_text("");

                // Create new conversation if none selected
                if self.current_conversation.is_none() {
                    if let Ok(new_conv) = self.db.create_conversation(&text) {
                        self.current_conversation = Some(new_conv);
                        let new_conversation = Conversation {
                            id: new_conv,
                            title: "New Conversation".to_string(),
                            created_at: Utc::now(),
                            updated_at: Utc::now(),
                        };
                        self.conversations.push(new_conversation.clone());

                        // Add to conversations list
                        let row = gtk::ListBoxRow::new();
                        let label = gtk::Label::new(Some(&new_conversation.title));
                        label.set_xalign(0.0);
                        row.set_child(Some(&label));
                        row.set_property("conversation-id", new_conversation.id);
                        // self.append(&row);
                    }
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

                // Save message to database
                if let Some(conv_id) = self.current_conversation {
                    let _ = self.db.add_message(
                        conv_id,
                        &chat_completion::ChatCompletionMessage {
                            role: chat_completion::MessageRole::user,
                            content: chat_completion::Content::Text(text.clone()),
                            name: None,
                            tool_calls: None,
                            tool_call_id: None,
                        },
                    );
                }

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
                        Ok(_) => {
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
                    Ok(_) => "No response".to_string(),
                    Err(e) => format!("ERROR: {}", e.to_string()),
                };

                self.messages.guard().push_back((text.clone(), false));

                self.chat_messages
                    .push(chat_completion::ChatCompletionMessage {
                        role: chat_completion::MessageRole::assistant,
                        content: chat_completion::Content::Text(text.clone()),
                        name: None,
                        tool_calls: None,
                        tool_call_id: None,
                    });

                // Save assistant message to database
                if let Some(conv_id) = self.current_conversation {
                    let _ = self.db.add_message(
                        conv_id,
                        &chat_completion::ChatCompletionMessage {
                            role: chat_completion::MessageRole::assistant,
                            content: chat_completion::Content::Text(text.clone()),
                            name: None,
                            tool_calls: None,
                            tool_call_id: None,
                        },
                    );
                }

                scroll_to_bottom();
            }
            AppMsg::ConversationSelected(conv_id) => {
                self.current_conversation = Some(conv_id);
                self.messages.guard().clear();
                self.chat_messages.clear();

                if let Ok(messages) = self.db.get_messages(conv_id) {
                    for msg in messages {
                        let is_user = msg.role == chat_completion::MessageRole::user;
                        self.chat_messages.push(msg.clone());
						match msg.content {
                            chat_completion::Content::Text(text) => {
                                self.messages.guard().push_back((text, is_user));
                                
                            }
                            _ => {
                                eprintln!("Received non-text message content");
                            }
                        };
                    }
                }
                scroll_to_bottom();
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.test.simple");

    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("style.css"));
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    } else {
        eprintln!("Failed to get default display");
    }
    app.run_async::<AppModel>(
        "
		You are a helpful LLM assistant, called Qwen, trained by Alibaba Cloud. 
		The specific version is Qwen2.5-7B-Instruct.
	"
        .to_string(),
    );
}

async fn get_llm_response(
    messages: &Vec<chat_completion::ChatCompletionMessage>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let api_key =
        env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY environment variable not set")?;

    let client = OpenAIClient::builder()
        .with_endpoint("https://api.siliconflow.cn/v1")
        .with_api_key(api_key)
        .build()
        .map_err(|e| format!("Failed to create API client: {}", e))?;

    let req = ChatCompletionRequest::new("Qwen/Qwen2.5-7B-Instruct".to_string(), messages.clone());

    let result = client
        .chat_completion(req)
        .await
        .map_err(|e| format!("API request failed: {}", e))?;

    if result.choices.is_empty() {
        return Err("No choices in API response".into());
    }

    Ok(result.choices[0].message.content.clone())
}
