use gtk::prelude::{WidgetExt, TextBufferExt, TextViewExt};
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::interpreter::{solve, ResType};
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

// Input component

pub struct LucaInput {
    text_buffer: gtk::TextBuffer
}

#[derive(Debug)]
pub enum MsgInput {
    TextChanged(String)
}

#[relm4::component(pub)]
impl SimpleComponent for LucaInput {
    type Init = String;
    type Input = ();
    type Output = MsgInput;

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

        text_buffer.connect_changed(move |text_buffer| {
            let start_iter = text_buffer.start_iter();
            let end_iter = text_buffer.end_iter();
            let text = text_buffer.text(&start_iter, &end_iter, false);

            // interpret the text from the input pane
            let mut results = String::new();
            let variables : Rc<RefCell<HashMap<String, ResType>>> = Rc::new(RefCell::new(HashMap::new()));
            
            for line in text.lines() {

                if let Ok(res) = solve(line.to_string(), variables.clone()) {
                    results.push_str(&res);
                    results.push_str("\n");
                } else {
                    results.push('\n');
                }
            }
            results.pop();

            sender.output(MsgInput::TextChanged(results.to_string())).unwrap();
        });

        let model = LucaInput {text_buffer};
        let widgets = view_output!();
        ComponentParts {model, widgets}
    }

    // fn update(&mut self, msgInput: Self::Input, _sender: ComponentSender<Self>) {
    //     match msg {
    //         Msg::TextChanged(text) => {
    //             self.text = text;
    //             if let Ok(res) = solve(self.text.clone()) {
    //                 println!("{}", res);
    //             }
    //         }
    //     }
    // }
}
