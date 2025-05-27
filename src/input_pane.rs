use gtk::prelude::{WidgetExt, TextBufferExt, TextViewExt};
use relm4::gtk::TextBuffer;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::interpreter::solve;
use crate::value::Value;
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

        create_tags(text_buffer.clone());

        text_buffer.connect_changed(move |text_buffer| {
            let start_iter = text_buffer.start_iter();
            let end_iter = text_buffer.end_iter();
            let text = text_buffer.text(&start_iter, &end_iter, false);

            // interpret the text from the input pane
            let mut results = String::new();
            let variables : Rc<RefCell<HashMap<String, Value>>> = Rc::new(RefCell::new(HashMap::new()));

            for (i, line) in text.lines().enumerate() {
                syntax_coloration(text_buffer.clone(), i, line);

                // Create the content of the Result Pane
                if let Ok(res) = solve(line.to_string(), variables.clone()) {
                    results.push_str(&res);
                }
                results.push('\n');
            }

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

fn create_tags(text_buffer: TextBuffer) {
        // Create and add the bold tag once
    let bold_tag = gtk::TextTag::builder()
        .name("bold")
        .weight(700) // bold in pango
        .build();
    text_buffer.tag_table().add(&bold_tag);
}

fn syntax_coloration(text_buffer: TextBuffer, line_number: usize, _line: &str) {

    if line_number == 0 {
        // Make the first line bold
        if let Some(start_iter) = text_buffer.iter_at_line_index(0, 0) {
            let mut end_iter = start_iter.clone();
            end_iter.forward_to_line_end();
            
            // Look up the tag by name
            if let Some(bold_tag) = text_buffer.tag_table().lookup("bold") {
                text_buffer.apply_tag(&bold_tag, &start_iter, &end_iter);
            }
        }
    }
}