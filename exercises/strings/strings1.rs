// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);

    // $str (string slice or view) becomes a pointer to a place in memory with a static length
    let data = "initial contents";

    // an imutable String struct 
    let s = data.to_string();

    // aStr is inferred as [&str; 3]
    let mut aStr = ["initial contents", "another", "another"];
    println!("My char array is {:?}", aStr);
    aStr[0] = "first string";
    println!("My char array is {:?}", aStr);

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}

fn current_favorite_color() -> String {
    String::from("blue")
}
