// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

// https://doc.rust-lang.org/reference/macros-by-example.html#the-macro_use-attribute
// #[macro_use] attribute can be used to make a module's macro
// scope not end when the module is closed.
#[macro_use]
mod macros {
    // Macros labeled with #[macro_export] are always pub and
    // can be referred to by other crates, either by path or
    // by #[macro_use] as described above.
    // #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
