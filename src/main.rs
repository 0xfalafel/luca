use gtk::{gdk, glib, glib::clone};
use gtk::prelude::{GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp, SimpleComponent};
use granite::prelude::SettingsExt;

mod input_pane;
use input_pane::LucaInput;

// Application model

struct AppModel {
    input: Controller<LucaInput>,
}

#[relm4::component]
impl SimpleComponent for AppModel {

    /// The type of the messages that this component can receive.
    type Input = ();
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = ();


    view! {
        main_window = gtk::Window {
            set_default_width: 600,
            set_default_height: 400,
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
                            pack_end = &gtk::WindowControls{
                                set_side: gtk::PackType::End,
                            },
                            add_css_class: "sidebar"
                        },
                        
                        gtk::Label {
                            set_label: "Hi mom!",
                            add_css_class: "sidebar"
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
        let text_input: Controller<LucaInput> = 
            LucaInput::builder()
                .launch(String::from("Hi mom!"))
                .forward(sender.input_sender(), |msg| match msg {
                    _ => {}
                });

        let model = AppModel {
            input: text_input
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
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

fn main() {

    let app = RelmApp::new("io.github.falafel.luca");
    app.run::<AppModel>(());
}