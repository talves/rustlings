// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    // Requiring a lifetime on the references tells the compiler
    // that the referenced values need to be kept around until they are no longer used in the Struct
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}
