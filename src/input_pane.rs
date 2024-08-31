use gtk::prelude::{WidgetExt, TextBufferExt, TextViewExt};
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::interpreter::solve;

// Input component

pub struct LucaInput {
    text: String,
    text_buffer: gtk::TextBuffer
}

#[derive(Debug)]
pub enum Msg {
    TextChanged(String)
}

#[relm4::component(pub)]
impl SimpleComponent for LucaInput {
    type Init = String;
    type Input = Msg;
    type Output = ();

    view! {
        gtk::TextView {
            set_margin_start: 20,
            set_buffer: Some(&model.text_buffer)
        },
    }

    fn init(
        text: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let text_buffer = gtk::TextBuffer::new(None);
        text_buffer.set_text(&text);

        let text_buffer_clone = text_buffer.clone();
        text_buffer_clone.connect_changed(move |text_buffer| {
            let start_iter = text_buffer.start_iter();
            let end_iter = text_buffer.end_iter();
            let text = text_buffer.text(&start_iter, &end_iter, false);
            sender.input(Msg::TextChanged(text.to_string()));
        });

        let model = LucaInput {text, text_buffer};
        let widgets = view_output!();
        ComponentParts {model, widgets}
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::TextChanged(text) => {
                self.text = text;
                if let Ok(res) = solve(self.text.clone()) {
                    println!("{}", res);
                }
                //println!("Text changed: {}", self.text);
            }
        }
    }
}
