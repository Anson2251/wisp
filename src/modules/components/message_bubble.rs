use gtk::prelude::*;
use relm4::factory::{FactoryComponent, FactorySender};
use relm4::prelude::DynamicIndex;
use relm4::gtk;

pub struct MessageBubble {
    text: String,
    is_user: bool,
}

#[derive(Debug)]
#[allow(unused)]
pub enum MessageBubbleMsg {
    UpdateText(String),
}

#[relm4::factory(pub)]
impl FactoryComponent for MessageBubble {
    type Init = (String, bool);
    type Input = MessageBubbleMsg;
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            set_halign: if self.is_user { gtk::Align::End } else { gtk::Align::Start },

            gtk::Label {
				#[watch]
                set_label: &self.text,
                set_wrap: true,
                set_max_width_chars: 40,
                set_css_classes: &["message-bubble", (if self.is_user { "user-bubble" } else {"llm-bubble"})],
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        MessageBubble {
            text: init.0,
            is_user: init.1,
        }
    }

	fn update(&mut self, message: Self::Input, _sender: FactorySender<Self>) {
		match message {
			MessageBubbleMsg::UpdateText(text) => {
				self.text = text;
			}
		}
	}
}
