// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit { message: String },
    Echo,
    Move,
    ChangeColor,
    FunNumber,
}

fn main() {
    println!(
        "{:?}",
        Message::Quit {
            message: "foo".to_string()
        }
    );
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
