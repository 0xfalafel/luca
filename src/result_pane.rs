use gtk::prelude::{WidgetExt, TextBufferExt, TextViewExt};
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::interpreter::solve;

// Input component

pub struct ResultView {
    text_buffer: gtk::TextBuffer
}

#[derive(Debug)]
pub enum ResultMsg {
    TextChanged(String)
}

#[relm4::component(pub)]
impl SimpleComponent for ResultView {
    type Init = String;
    type Input = ResultMsg;
    type Output = ();

    view! {
        gtk::TextView {
            set_margin_start: 20,
            set_editable: false,
            set_buffer: Some(&model.text_buffer)
        },
    }

    fn init(
        text: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let text_buffer = gtk::TextBuffer::new(None);
        text_buffer.set_text(&text);

        let model = ResultView {text_buffer};
        let widgets = view_output!();
        ComponentParts {model, widgets}
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ResultMsg::TextChanged(text) => {
                // println!("input: {}", &text);
                
                let mut results = String::new();

                for line in text.lines() {

                    if let Ok(res) = solve(line.to_string()) {
                        results.push_str(&res);
                        results.push_str("\n");
                    } else {
                        results.push('\n');
                    }
                }
                results.pop();
                self.text_buffer.set_text(&results);
            }
        }
    }
}
