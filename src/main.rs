mod data;

use pipewire::{context::ContextRc, main_loop::MainLoopRc, types::ObjectType};

use crate::data::config_handler::{Config, ConfigHandler, Data};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new(
        "config".to_string(),
        "config.json".to_string(),
        Data::new(vec![]),
    );
    let mut config_handler = ConfigHandler::new(config);

    match config_handler.load() {
        Ok(()) => {}
        Err(e) if e.downcast_ref::<std::io::Error>().is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) => {}
        Err(e) => return Err(e),
    }
    config_handler.save()?;

    println!("{:?}", serde_json::to_string(&config_handler.data()));

    Ok(())
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mainloop = MainLoopRc::new(None)?;
//     let context = ContextRc::new(&mainloop, None)?;
//     let core = context.connect_rc(None)?;
//     let registry = core.get_registry_rc()?;

//     let _listener = registry
//         .add_listener_local()
//         .global(|global| {
//             if global.type_ != ObjectType::Node {
//                 return;
//             }

//             println!("{:?}", global);

//             let Some(props) = &global.props else {
//                 return;
//             };

//             let name = props.get("node.name").unwrap_or("Unknown");
//             let description = props.get("node.description").unwrap_or(name);

//             let media_class = props.get("media.class").unwrap_or("Unknown");

//             match media_class {
//                 "Audio/Sink" => {
//                     println!("Output: {description} ({name})");
//                 }
//                 "Audio/Source" => {
//                     println!("Input: {description} ({name})");
//                 }
//                 "Stream/Output/Audio" => {
//                     println!("Application: {description} ({name})");
//                 }
//                 _ => {}
//             }
//         })
//         .register();

//     println!("soundpulse pipewire manager started.");

//     mainloop.run();

//     Ok(())
// }
