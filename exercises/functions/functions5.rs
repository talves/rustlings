// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

fn main() {
    let answer: i32 = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
    // num * num; the semicolon makes it return an empty tuple
}
