use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// We don't have to implement 'impl HelloMacro for Pancakes'
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
