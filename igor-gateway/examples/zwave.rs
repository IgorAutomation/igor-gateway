use std::io;
use std::io::Write;
use igor_gateway::zwave::ZWaveController;

fn display_prompt() {
    print!("> ");
    io::stdout().flush().unwrap(); // https://github.com/rust-lang/rust/issues/23818
}

fn main() {

    let controller = ZWaveController::new(String::from("/dev/cu.usbmodem141401"));

    println!("Enter `exit` or Control-C to exit.");
    let mut input = String::new();
    loop {
        input.clear();
        display_prompt();

        if let Ok(n) = io::stdin().read_line(&mut input) {
            if n == 0 {
                // End-of-file (either Control-D or we were redirected).
                break;
            }
        } else {
            println!("Error reading stdin");
            break;
        }

        let tokens: Vec<_> = input.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        println!("read token: {}", tokens[0]);

        match tokens[0] {
            "exit" | "q" | "quit" | "fuck" => break,
            "devices" => {
                controller.print_nodes();
            },
            "controller" => {
                controller.print_controller_information();
            },
            "homes" => {
                for id in controller.get_home_ids() {
                    println!("home id: {}", id);
                }
            },
            _ => println!("unrecognized input {}", tokens[0])
        }
    }
    println!("exiting.");
}
