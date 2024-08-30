#![allow(unused)]

use gtk::{gdk, glib::clone, HeaderBar, PackType};
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

struct AppModel;

#[relm4::component]
impl SimpleComponent for AppModel {

    /// The type of the messages that this component can receive.
    type Input = ();
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = ();
    /// The root GTK widget that this component will create.
    //type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    // type Widgets = AppWidgets;
    //type Widgets = ();


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

                        gtk::TextView {
                            set_margin_start: 20
                            
                        }
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
        counter: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        load_css();

        let model = AppModel {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    // fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
    // }

    // /// Update the view to represent the updated model.
    // fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
    // }
}

// fn initialize_custom_style() {
//     gio::resources_register_include!("style.css").unwrap();

//     let display = gdk::Display::default().unwrap();
//     let theme = gtk::IconTheme::for_display(&display);
//     theme.add_resource_path("/com/example/Foobar/icons");
// }

fn load_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    provider.load_from_data(include_str!("../data/style.css"));
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}

fn main() {
    let app = RelmApp::new("io.github.falafel.luca");
    app.run::<AppModel>(());
}