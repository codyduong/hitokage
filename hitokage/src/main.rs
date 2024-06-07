use chrono::Local;
use gtk4::prelude::*;
use hitokage_core::common::EventNotif;
use hitokage_core::common::EVENT;
use hitokage_core::common::NEW_EVENT;
use hitokage_lua::AppMsg;
use hitokage_lua::LuaHookType;
use relm4::prelude::*;
use std::fs::File;
use std::io::Read;
mod bar;
mod socket;
mod win_utils;

const HITOKAGE_STATUSBAR_HEIGHT: i32 = 64;

struct App {
  current_time: String,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
  type Input = hitokage_lua::AppMsg;
  type Output = ();
  type Init = ();

  view! {
    gtk::ApplicationWindow {
      set_visible: false,

      // LOL! Is there a better way to prevent this from showing?
      connect_show => move |window| {
        window.set_visible(false)
      }
    },
  }

  fn init(_: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    // start the lua hook

    let lua_file_path = "./hitokage.lua";
    let mut file = File::open(lua_file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lua_script = contents;

    let sender_clone = sender.clone();
    let _lua_thread = std::thread::spawn(move || {
      let lua = hitokage_lua::make(sender_clone).unwrap();

      let co = lua
        .create_thread(lua.load(lua_script).into_function().unwrap())
        .unwrap();

      loop {
        match co.resume::<_, ()>(()) {
          Ok(_) => (),
          Err(err) => {
            println!("Lua error: {:?}", err);
            break;
          }
        }
        // std::thread::sleep(std::time::Duration::from_millis(100)); // Small sleep to prevent tight loop
      }
    });

    // komorebi pipe
    let sender_clone = sender.clone();
    socket::start_async_reader_new(sender_clone);

    // system clock
    let sender_clone = sender.clone();
    // "Precise timing is not guaranteed, the timeout may be delayed by other events."
    // so yeah, use 500ms increment, if we skip a second we have bigger problems performance wise...
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
      sender_clone.input(AppMsg::Tick);
      glib::ControlFlow::Continue
    });

    // bar.widget().realize();
    // gtk4::prelude::WidgetExt::realize(bar.widget());

    let model = App {
      current_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
      // bar,
    };

    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: AppMsg, sender: ComponentSender<Self>) {
    match msg {
      // Komorebi States
      AppMsg::Komorebi(notif) => {
        // println!("{:?}", &notif);
        let notif: Option<EventNotif> = serde_json::from_str(&notif).unwrap_or_else(|err| {
          log::warn!("Failed to read notification from komorebic: {:?}", err);

          None
        });

        // Breaks on version mismatch between built and available version...
        // let notif: Option<komorebi_client::Notification> = serde_json::from_str(&notif).unwrap_or_else(|err| {
        //   log::warn!("Failed to read notifcations from komorebic: {:?}", err);

        //   None
        // });

        if let Some(notif_value) = notif {
          // let mut result = {
          //   let rg = STATE.read();
          //   let result = rg.to_vec();
          //   drop(rg);
          //   result
          // };
          // result.push(notif_value);

          // let mut r = Vec::new();
          // r.push(notif_value);

          // *STATE.write() = r;
          let mut sswg = EVENT.write();
          sswg.push(notif_value);
          drop(sswg);
          *NEW_EVENT.write() = true;
        }

        // println!("{:?}", &notif);
        // self.output = String::new();
        // self.output.push_str(&notif);
        // self.output.push('\n');
      }
      AppMsg::KomorebiErr(line) => {
        println!("{:?}", &line);
        // self.output = String::new();
        // self.output.push_str("ERROR!");
        // self.output.push_str(&line);
        // self.output.push('\n');
      }

      //
      AppMsg::LuaHook(info) => match info.t {
        LuaHookType::CreateBar(props) => {
          let app = relm4::main_application();
          let builder = bar::Bar::builder();

          // app.add_window(&builder.root);

          app.add_window(&builder.root);

          let bar = builder
            .launch(props)
            .forward(sender.input_sender(), std::convert::identity);

          ()
        }
        LuaHookType::ReadEvent => {
          *crate::EVENT.write() = Vec::new();
          *crate::NEW_EVENT.write() = false;

          ()
        }
        LuaHookType::NoAction => (),
        _ => {
          println!("todo");

          ()
        }
      },

      // Primary Program Loop
      AppMsg::Tick => {
        self.current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
      }
    }
  }
}

#[tokio::main]
async fn main() {
  simple_logger::SimpleLogger::new().init().unwrap();

  let style_file_path = "./styles.css";

  let app = RelmApp::new("com.example.hitokage");
  let _ = app.set_global_css_from_file(style_file_path);
  app.run::<App>(());
}
