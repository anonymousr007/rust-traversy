use std::env;

// Outputs 
// `Command`               Output 
// `cargo run`             Args: ["target/debug/rust"]
// `cargo run hello`       Args: ["target/debug/rust", "hello"]
// `cargo run hello world` Args: ["target/debug/rust", "hello", "world"]

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Brad";
    let status = "100%";

    // println!("Args: {:?}", args);
    // println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name); // `cargo run hello` 
    } else if command == "status" {
        println!("Status is {}", status); // `cargo run status`
    } else {
        println!("That is not a valid comment");
    }
}