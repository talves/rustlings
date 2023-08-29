// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut x = 100;
    let y = &mut x;
    // dereferences the value y to mut x
    *y += 100;
    // println!("x: {}", x);
    let z = &mut x;
    // dereferences the value z to mut x
    *z += 1000;
    // println!("x: {}", x);
    assert_eq!(x, 1200);
}
