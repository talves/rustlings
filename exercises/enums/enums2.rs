// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(u32, u32, u32),
}

impl Message {
    fn call(&self) {
        match self {
            Self::Echo(message) => println!("{}", message),
            Self::ChangeColor(r, g, b) => println!("Red: {}, Green: {}, and Blue: {}!", r, g, b),
            Self::Move { x, y } => println!("Moving x: {} y: {}", x, y),
            Self::Quit => println!("Quitting"),
        }
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
