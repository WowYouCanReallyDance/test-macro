use runnable::test_a::how_are_you;
use test_macro::{auto_call, expand_funcs};

mod runnable;

#[auto_call]
fn good_good_study() {
    println!("Good Good Study! Day Day Up!");
}

fn main() {
    expand_funcs!();
}
