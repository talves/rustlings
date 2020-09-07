// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

// Everything is private in Rust by default

mod sausage_factory {
    pub fn make_sausage() {
        println!("sausage!");
    }
}

fn main() {
    // crate::sausage_factory::make_sausage();
    sausage_factory::make_sausage();
}
