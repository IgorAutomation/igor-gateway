extern crate openzwave_stateful as openzwave;
use openzwave::{ ConfigPath, InitOptions };
use openzwave::{ ZWaveNotification };
use std::sync::{mpsc, Arc};
use std::{thread};
use self::openzwave::{ZWaveManager};


pub struct ZWaveController {
    ozw: Arc<ZWaveManager>
}

///
/// ZWave controller.
///
impl ZWaveController {

    ///
    /// Spawn notifications
    ///
    fn spawn_notifications(rx: mpsc::Receiver<ZWaveNotification>) {
        thread::spawn(move || {
            for notification in rx {
                println!("zwave notification received: {}", notification);
            }
        });
    }

    ///
    /// Create an instance of our controller
    ///
    pub fn new(device: String) -> Self {
        let mut device_args = Vec::new();
        device_args.push(device);
        let options = InitOptions {
            devices: Some(device_args),
            config_path: ConfigPath::Default,
            user_path: "./" // TODO: parameterize
        };

        let (ozw, rx) = openzwave::init(&options).unwrap();

        let ret = ZWaveController {
            ozw
        };

        ZWaveController::spawn_notifications(rx);

        return ret;
    }

    pub fn print_controller_information(&self) {
        let state = self.ozw.get_state();
        let controllers = state.get_controllers();
        for (controller, info) in controllers {
            println!("{} {}", controller, info);
        }
    }

    ///
    /// Get a list of home ids.
    ///
    pub fn get_home_ids(&self) -> Vec<u32> {
        let mut ret = Vec::new();
        let state = self.ozw.get_state();
        let controllers = state.get_controllers();
        for (controller, _info) in controllers {
            ret.push(controller.get_home_id())
        }

        return ret;
    }

    ///
    /// Print information about found nodes.
    ///
    pub fn print_nodes(&self) {
        let state = self.ozw.get_state();
        let node_map = state.get_nodes_map();
        for (controller, node_set) in node_map {
            let info_str = state.get_controller_info(controller).map_or(String::from("???"), |info| info.to_string());
            println!("{} {}", controller, info_str);
            for node in node_set.iter() {
                println!("  Node: {}", node);
            }
        }
    }
}
