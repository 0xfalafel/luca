use gtk::prelude::{WidgetExt, TextBufferExt, TextViewExt};
use luca::interpreter::{syntax_analysis, Token};
use relm4::gtk::TextBuffer;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::interpreter::solve;
use crate::value::Value;
use std::collections::HashMap;
use std::cell::RefCell;
use std::i32;
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

            // Remove all the coloration tags, since we reapply them on the newest input
            text_buffer.remove_all_tags(&start_iter, &end_iter);

            // interpret the text from the input pane
            let mut results = String::new();
            let variables : Rc<RefCell<HashMap<String, Value>>> = Rc::new(RefCell::new(HashMap::new()));

            for (i, line) in text.lines().enumerate() {
                syntax_coloration(text_buffer.clone(), i as i32, line);

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
    let tag_table = text_buffer.tag_table();

    // Bold
    let bold_tag = gtk::TextTag::builder()
        .name("bold")
        .weight(700) // bold in pango
        .build();
    tag_table.add(&bold_tag);

    // Number
    let number_tag = gtk::TextTag::builder()
        .name("number")
        .foreground("#3689e6")
        .build();
    tag_table.add(&number_tag);

    // Unit
    let unit_tag = gtk::TextTag::builder()
        .name("unit")
        .foreground("#a56de2")
        .build();
    tag_table.add(&unit_tag);

}

fn syntax_coloration(text_buffer: TextBuffer, line_number: i32, line: &str) {

    let tokens = syntax_analysis(line);

    // For debug pupropses
    // for (token, start, end) in tokens.clone() {
    //     println!("token: {:?}, start: {}, end: {}", token, start, end);
    // }
    // println!("----------------------------------------------------");

    for (token, start, end) in tokens {
        match token {
            Token::TITLE => {
                if let Some(start_iter) = text_buffer.iter_at_line_index(line_number, 0) {
                    let mut end_iter = start_iter.clone();
                    end_iter.forward_to_line_end();
                    
                    // Look up the tag by name
                    if let Some(bold_tag) = text_buffer.tag_table().lookup("bold") {
                        text_buffer.apply_tag(&bold_tag, &start_iter, &end_iter);
                    }
                }               
            },
            Token::NUMBER(_) => {
                apply_tag(text_buffer.clone(), line_number, start as i32, end as i32, "number");
            },
            Token::UNIT(_) | Token::PERCENTAGE | Token::MONEY(_) => {
                apply_tag(text_buffer.clone(), line_number, start as i32, end as i32, "unit");
            },
            _ => {}
        }
    }
}

fn apply_tag(text_buffer: TextBuffer, line: i32, start: i32, end: i32, tag_name: &str) {

    if let Some(start_iter) = text_buffer.iter_at_line_offset(line, start) {

        if let Some(end_iter) = text_buffer.iter_at_line_offset(line, end) {
            
            // Look up the tag by name
            if let Some(bold_tag) = text_buffer.tag_table().lookup(tag_name) {
                text_buffer.apply_tag(&bold_tag, &start_iter, &end_iter);
            }            
        }
    }
}