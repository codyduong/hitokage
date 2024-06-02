use chrono::Local;
use gtk4::gdk;
use gtk4::prelude::*;
use log::trace;
use relm4::prelude::*;
use windows::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

mod pipes;
mod win_utils;

#[derive(Debug)]
enum Msg {
    Komorebi(String),
    KomorebiErr(String),
    Tick, // system clock
}

struct Model {
    output: String,
    current_time: String,
}

const HITOKAGE_STATUSBAR_HEIGHT: i32 = 64;

#[relm4::component]
impl SimpleComponent for Model {
    type Input = Msg;
    type Output = ();
    type Init = ();
    type Widgets = AppWidgets;

    view! {
        gtk::ApplicationWindow {
            set_default_size: (0, 0),
            set_resizable: false,
            set_display: &gdk::Display::default().expect("Failed to get default display"),
            set_decorated: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                gtk::Label {
                    set_label: "Hello, World!",
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("{}", model.current_time),
                },

                gtk::Label {
                    set_natural_wrap_mode: gtk::NaturalWrapMode::Word,

                    // #[watch]
                    // set_label: &format!("{}", model.output.as_str()),
                },

                gtk::Button {
                    set_label: "Test",
                    connect_clicked => move |window| {
                        println!("foobar");
                    }
                },
            },

            connect_realize => move |window| {
                let x = win_utils::get_primary_width();
                window.set_size_request(x, HITOKAGE_STATUSBAR_HEIGHT);
            },

            // this fails in realize, for what reason i have no clue LOL!
            connect_show => move |window| {
                // Set Status bar to TOP or BOTTOM of screen

                // https://discourse.gnome.org/t/set-absolut-window-position-in-gtk4/8552/4
                let native = window.native().expect("Failed to get native");
                let surface = native.surface().expect("Failed to get surface");

                // specifically for windows -> https://discourse.gnome.org/t/how-to-center-gtkwindows-in-gtk4/3112/13
                let handle = surface.downcast::<gdk4_win32::Win32Surface>().expect("Failed to get Win32Surface").handle();
                let win_handle = HWND(handle.0);

                println!("{:?}", win_handle);

                unsafe {
                    SetWindowPos(
                        win_handle,
                        HWND_TOPMOST,
                        0,
                        0,
                        0,
                        0,
                        SWP_NOSIZE,
                    )
                    .ok();
                }
            }
        }
    }

    fn init(
        _init_params: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Model {
            current_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            output: String::new(),
        };

        // Insert the code generation of the view! macro here
        let widgets = view_output!();

        // komorebi pipe
        let sender_clone = sender.clone();
        pipes::start_async_reader(sender_clone);

        // system clock
        let sender_clone = sender.clone();
        // "Precise timing is not guaranteed, the timeout may be delayed by other events."
        // so yeah, use 500ms increment, if we skip a second we have bigger problems performance wise...
        glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
            sender_clone.input(Msg::Tick);
            glib::ControlFlow::Continue
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Msg, _sender: ComponentSender<Self>) {
        match msg {
            Msg::Komorebi(notif) => {
                // println!("{:?}", &notif);
                self.output = String::new();
                self.output.push_str(&notif);
                // self.output.push('\n');
            }
            Msg::KomorebiErr(line) => {
                self.output = String::new();
                self.output.push_str("ERROR!");
                self.output.push_str(&line);
                // self.output.push('\n');
            }
            Msg::Tick => {
                self.current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            }
        }
    }
}

mod lua;

fn main() {
    let lua = lua::make_lua().unwrap();

    simple_logger::SimpleLogger::new().init().unwrap();

    let lua_result = lua
        .load(
            r#"
print(hitokage);
hitokage.print("foobarbaz")
print("lit");
"#,
        )
        .exec();

    println!("{:?}", lua_result);

    let app = RelmApp::new("com.example.Relm4App");
    app.run::<Model>(());
}
