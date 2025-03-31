# Basic
Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming. However, unlike macros in C and other languages, Rust macros are expanded into abstract syntax trees, rather than string preprocessing.
&nbsp;

The term *macro* refers to a family of features in Rust: ***declarative* macros** with `macro_rules!` and three kinds of ***procedural* macros**:
&nbsp;
- Custom `#[derive]` macros that specify code added with the `derive` attribute used on structs and enums
- Attribute-like macros that define custom attributes usable on any item
- Function-like macros that look like function calls but operate on the tokens specified as their argument
&nbsp;

# Advanced
## tokenization
The first stage of compilation for a Rust program is tokenization. This is where the source text is transformed into a sequence of tokens:
- Identifiers: foo, Bambous, self, we_can_dance, …
- Literals: 42, 72u32, 0_______0, 1.0e-40, "squirrels", …
- Keywords: _, fn, self, match, yield, macro, gen, …
- Symbols: [, :, ::, ?, ~, @1, …
## Parsing
The next stage is parsing, where the stream of tokens is turned into an [Abstract Syntax Tree (AST)](https://en.wikipedia.org/wiki/Abstract_syntax_tree).
Before the AST, we call the tokens the token tree.
## Syntax Extensions
`macro` processing in Rust happens after the construction of the AST. As such, the syntax used to invoke a macro must be a proper part of the language's syntax. 
1. `# [ $arg ]`; *e.g.* `#[derive(Clone)]`, `#[no_mangle]`, …
2. `# ! [ $arg ]`; *e.g.* `#![allow(dead_code)]`, `#![crate_name="blang"]`, …
3. `$name ! $arg`; *e.g.* `println!("Hi!")`, `concat!("a", "b")`, …
4. `$name ! $arg0 $arg1`; *e.g.* `macro_rules! dummy { () => {}; }`.

The first two are [attributes](https://doc.rust-lang.org/reference/attributes.html) which annotate items, expressions and statements. They can be classified into different kinds, [built-in attributes](https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index), [proc-macro attributes](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros) and [derive attributes](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macro-helper-attributes). [proc-macro attributes](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros) and [derive attributes](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macro-helper-attributes) can be implemented with the second macro system that Rust offers, [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html). [built-in attributes](https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index) on the other hand are attributes implemented by the compiler.
## Expansion
At some point after the construction of the AST, but before the compiler begins constructing its semantic understanding of the program, it will expand all syntax extensions. Expansion happens in "passes"; as many as is needed to completely expand all invocations.

This is known as the syntax extension recursion limit and defaults to 128. If the 128th expansion contains a syntax extension invocation, the compiler will abort with an error indicating that the recursion limit was exceeded.

This limit can be raised using the #![recursion_limit="…"] attribute, though it must be done crate-wide.

## Hygiene
Hygiene describes the ability for a macro to work in its own syntax context, not affecting nor being affected by its surroundings.

## `span`
The type in [proc_macro](https://doc.rust-lang.org/proc_macro/struct.Span.html) library that encodes Hygiene feature for procedural macros.

Every token in a TokenStream has an associated Span holding some additional info. A span, as its documentation states, is A region of source code, along with macro expansion information.

# Syntax
## Declarative Macros
### Defining by Designators
The arguments of a macro are prefixed by a dollar sign `$` and type annotated with a `*designator*`:

```rust
macro_rules! create_function {
    // The `name` designator is used for variable/function names.
    ($func_name:name) => {
        fn $func_name() {
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}
create_function!(foo);
```

The `designator` can also be in forms of expressions or closures.
###  Defining by Overloading
Macros can be overloaded to accept different combinations of arguments. 
```rust
macro_rules! operation {
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
}
operation!(true; or false);
operation!(true; and false);
```
###  Defining with Repetitions
Macros can use + in the argument list to indicate that an argument may repeat at least once, or *, to indicate that the argument may repeat zero or more times.
```rust
macro_rules! add {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => (
        $x + add!($($y),+)
    )
}
println!("{}", add!(1));
println!("{}", add!(1 + 2, 2));
println!("{}", add!(5, 2 * 3, 4));

```
```rust
// The defination of println!
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::io::_print($crate::format_args_nl!($($arg)*));
    }};
}
```
## Procedural Macros
### Custom `#[derive]` Macro
Please see the example in `/derive_macro_example` and the explanation here: [How to Write a Custom derive Macro](https://doc.rust-lang.org/book/ch20-06-macros.html#how-to-write-a-custom-derive-macro)

### Attribute-like Macro
```rust
#[proc_macro_attribute]
pub fn my_attribute(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[my_attribute(my_attr)]
fn foo(){}
```

### Function-like Macro
Unlike with declarative macros though, function-like procedural macros do not have certain restrictions imposed on their inputs though. 
```rust
use proc_macro::TokenStream;

#[proc_macro]
pub fn my_proc_macro(input: TokenStream) -> TokenStream {
    input
}

fn foo() {
    my_proc_macro!(squirrel with a gun);
}
```


# Reference
- [The Rust Book - Macros](https://doc.rust-lang.org/book/ch20-06-macros.html#macros)
- [Rust by Example - macro_rules!](https://doc.rust-lang.org/rust-by-example/macros.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
- [proc_macro](https://doc.rust-lang.org/proc_macro/index.html)

# Resources
- [macrokata](https://github.com/tfpk/macrokata)
- [Rustlings](https://github.com/rust-lang/rustlings/tree/main/exercises/21_macros)