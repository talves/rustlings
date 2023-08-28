// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me();
    call_me_without();
}

// Use unit type as the function return type when NO return value
fn call_me() -> () {
    println!("Inside call_me");
}

// Can just leave the return type off and it will use the `unit type` as return type
fn call_me_without() {
    println!("Inside call_me_without");
}
