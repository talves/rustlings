// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

// I AM DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    // for x in option {
    //     res += x;
    // }
    println!("{}", res);
}
