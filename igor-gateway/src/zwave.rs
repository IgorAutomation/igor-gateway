extern crate openzwave_stateful as openzwave;
use openzwave::{ ConfigPath, InitOptions };
use openzwave::{ ValueGenre, ValueID, ZWaveNotification };

fn main() {

    // i literally have no idea what i am doing
    let options = InitOptions {
        devices: if device_args.len() == 0 { None } else { Some(device_args) },
        config_path: ConfigPath::Default,
        user_path: "./config/", // TODO: we need a directory to use i think
    };

    let (ozw, rx) = openzwave::init(&options).unwrap();

    let state = ozw.get_state();
    let controllers = state.get_controllers();
    for (controller, info) in controllers {
        println!("{} {}", controller, info);
    }
}
