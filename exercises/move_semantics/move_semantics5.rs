// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

// Note that a reference’s scope starts from where it is introduced and
// continues through the last time that reference is used.
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100; // end of y's scope
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
