use test_macro::auto_call;

#[auto_call]
pub fn how_are_you() {
    println!(
        "
    How are you?
    How are you?
    How are you today?
    I'm fine thanks!
    I'm fine thanks!
    Have a lovely day!
    "
    );
}
