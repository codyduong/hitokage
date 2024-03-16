// mod config;
// use config::read_config_main;
mod pipes;
use cxxqt_object::KomorebiPipeRust;
use pipes::{create_and_connect_pipe, read_from_pipe};

pub mod cxxqt_object;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
#[tokio::main]
async fn main() {
    // read_config_main();

    // QCoreApplication::init(|app| {
    //     let mut engine: cxx::UniquePtr<QQmlApplicationEngine> = QQmlApplicationEngine::new();
    //     let rust_object = QBox::new(MyRustObject::new());

    //     // Komorebi Pipe
    //     let pipe_handler = my_rust_object.clone();
    //     tokio::spawn(async move {
    //         if let Ok(pipe) = create_and_connect_pipe().await {
    //             read_from_pipe(pipe, pipe_handler).await;
    //         } else {
    //             println!("Failed to create or connect to the named pipe");
    //         }
    //     });

    //     // Expose the Rust object to QML
    //     engine
    //         .root_context()
    //         .set_context_property("myRustObject", &rust_object);

    //     engine.load(&QUrl::from("qrc:/qt/qml/hitokage/qml/main.qml"));

    //     app.exec()
    // });

    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // let pipe_handler = komorebi_pipe_obj.clone();
    tokio::spawn(async move {
        if let Ok(pipe) = create_and_connect_pipe().await {
            println!("{:?}", read_from_pipe(pipe).await.ok());
        } else {
            println!("Failed to create or connect to the named pipe");
        }
    });

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/hitokage/qml/main.qml"));
    }

    // ? https://github.com/KDAB/cxx-qt/blob/4e8c0ff412a5f65998b79eada1ef73c1080f6a3e/examples/cargo_without_cmake/src/main.rs#L31-L35
    // if let Some(engine) = engine.as_mut() {
    //     // Listen to a signal from the QML Engine
    //     engine.as_qqmlengine().on_quit(|_| {
    //         println!("QML Quit!");
    //     });
    // }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
