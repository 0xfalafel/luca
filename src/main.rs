use gtk::{gdk, glib, glib::clone};
use gtk::prelude::{GtkWindowExt, OrientableExt, WidgetExt};
use relm4::gtk::gio;
use log::{error, info};
use relm4::gtk::prelude::AdjustmentExt;
use relm4::{gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp, SimpleComponent};
use granite::prelude::SettingsExt;

mod input_pane;
use input_pane::{LucaInput, MsgInput};

mod result_pane;
use result_pane::{ResultView, ResultMsg};

// Things needed for --cli
use clap::Parser;
use value::Value;
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, ErrorKind, Read};
use std::io::Write;

mod interpreter;
mod units;
mod value;
use crate::interpreter::solve;

// Application model
#[derive(Debug)]
enum AppMsg {
    TextChanged(String)
}

struct AppModel {
    input: Controller<LucaInput>,
    result: Controller<ResultView>
}

#[relm4::component]
impl SimpleComponent for AppModel {

    /// The type of the messages that this component can receive.
    type Input = AppMsg;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = ();


    view! {
        main_window = gtk::Window {
            set_default_width: 600,
            set_default_height: 400,
            set_width_request: 370,
            set_title: Some(""),
            set_titlebar: Some(&gtk::Grid::new()), // set an emply headerbar

            gtk::Paned {
                set_orientation: gtk::Orientation::Horizontal,

                #[wrap(Some)]
                set_start_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_size_request: (250, -1),
                    gtk::HeaderBar {
                        set_show_title_buttons: false,
                        pack_start = &gtk::WindowControls{},
                        add_css_class: "view",
                    },

                    gtk::ScrolledWindow {
                        set_vexpand: true,
                        add_css_class: "view",
                        add_css_class: "text",
                        
                        set_child: Some(model.input.widget())
                    }
                },

                #[wrap(Some)]
                set_end_child = &gtk::WindowHandle {
                    gtk::Box {
                        set_vexpand: true,
                        add_css_class: "sidebar",
                        set_orientation: gtk::Orientation::Vertical,
                        gtk::HeaderBar {
                            set_show_title_buttons: false,
                            set_margin_start: 5,
                            pack_end = &gtk::Box{
                                set_orientation: gtk::Orientation::Horizontal,

                                gtk::MenuButton {
                                    set_icon_name: "preferences-system-symbolic",
                                    set_popover: Some(&gtk::PopoverMenu::from_model(Some(&{
                                        let menu = gio::Menu::new();
                                        menu.append(Some("Preferences"), Some("app.preferences"));
                                        menu
                                    })))
                                },

                                gtk::WindowControls {
                                    set_side: gtk::PackType::End,
                                }
                            },
                            add_css_class: "sidebar"
                        },
                        
                        gtk::ScrolledWindow {
                            set_vexpand: true,
                            add_css_class: "view",
                            add_css_class: "text",
                            set_child: Some(model.result.widget())
                        }
                    }
                },
            },
        }
    }

    /// Initialize the UI and model.
    fn init(
        _params: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        load_css();

        let initial_notebook = load_notebook();

        let text_input: Controller<LucaInput> = 
            LucaInput::builder()
                .launch(initial_notebook)
                .forward(sender.input_sender(), |msg| match msg {
                    MsgInput::TextChanged(new_text) => {AppMsg::TextChanged(new_text)}
                });

        let result_view: Controller<ResultView> = 
            ResultView::builder()
                .launch(String::from(""))
                .detach();


        // Sync both panes when scrolling
        let adjustment_input = text_input.widget().vadjustment();
        let adjustment_result = result_view.widget().vadjustment();

        // Input -> Result
        adjustment_input.connect_value_changed({
            let adjustment_res = adjustment_result.clone();
            move |adj| {
                adjustment_res.set_value(adj.value());
            }
        });

        // Result -> Input
        adjustment_result.connect_value_changed({
            let adjustment_in = adjustment_input.clone();
            move |adj| {
                adjustment_in.set_value(adj.value());
            }
        });
        
        let model = AppModel {
            input: text_input,
            result: result_view
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::TextChanged(new_text) => {
                self.result.emit(ResultMsg::TextChanged(new_text))
            }
        }
    }
}

// from https://jamesbenner.hashnode.dev/how-to-style-your-gtk4-rust-app-with-css
fn load_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    // load our custom CSS
    provider.load_from_data(include_str!("../data/style.css"));
    gtk::style_context_add_provider_for_display(&display, &provider, priority);


    // from https://github.com/davidmhewitt/elementary-rust-example/blob/main/src/application.rs#L81

    // follow dark theme if present
    if let Some(gtk_settings) = gtk::Settings::default() {
 
        granite::init();
        if let Some(granite_settings) = granite::Settings::default() {
            
            // Use the dark theme, if it's the theme prefered globaly
            gtk_settings.set_gtk_application_prefer_dark_theme(
                granite_settings.prefers_color_scheme() == granite::SettingsColorScheme::Dark
            );
            
            // Auto switch theme when the preferences are changed
            granite_settings.connect_prefers_color_scheme_notify(
                clone!(@weak gtk_settings => move |granite_settings| {
                    gtk_settings.set_gtk_application_prefer_dark_theme(
                        granite_settings.prefers_color_scheme() == granite::SettingsColorScheme::Dark
                    );
                })
            );
        }
    }
}


// We support a --cli argument to run the app in the terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	// CLI mode
    #[arg(short, long, action = clap::ArgAction::SetTrue, help = "Run in cli mode")]
	cli: bool,
}

/// Load a notebook file
fn load_notebook() -> String {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("pro.lasne.luca");
    if let Some(notebook_path) = xdg_dirs.get_data_file("previous_notebook.md") {
        match File::open(&notebook_path) {
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => info!("No previous notebook ({})", notebook_path.display()),
                    _ => error!("Failed to open notebook {}: {:#?}", notebook_path.display(), e.kind())
                }
            },
            Ok(mut notebook) => {
                let mut notebook_content = String::new();
                let res = notebook.read_to_string(&mut notebook_content);

                if res.is_err() {
                    error!("Failed to read notebook {} content: {}", notebook_path.display(), res.unwrap_err())                    
                }

                return notebook_content;
            },
        }
    }
    
    String::from("")
}

/// CLI mode: we create a small interpreter without launching the UI
fn cli() {
    let variables: Rc<RefCell<HashMap<String, Value>>> = Rc::new(RefCell::new(HashMap::new()));

    loop {
        // show the interactive prompt
        print!("calc> ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
    
        // read input from user
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.eq("") || input.eq("exit\n") {
            break;
        }

        match solve(input, variables.clone()) {
            Ok(result) => println!("{}", result),
            Err(_) => println!("Invalid syntax")
        }
    }
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    if args.cli {
        cli();
        return;
    }

    let app = RelmApp::new("pro.lasne.luca");
    app.run::<AppModel>(());
}