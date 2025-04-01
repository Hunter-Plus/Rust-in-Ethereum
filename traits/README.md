# Basic

## Definition

A `trait` defines the functionality a particular type has and can share with other types.
A `trait` is a collection of methods defined for an unknown type: `Self`.
A `trait` describes an abstract interface that types can implement. 

This interface consists of [associated items](https://doc.rust-lang.org/reference/items/associated-items.html), which come in three varieties:

- [functions](https://doc.rust-lang.org/reference/items/associated-items.html#associated-functions-and-methods)
- [types](https://doc.rust-lang.org/reference/items/associated-items.html#associated-types)
- [constants](https://doc.rust-lang.org/reference/items/associated-items.html#associated-constants)

Generic items may use traits as [bounds](https://doc.rust-lang.org/reference/trait-bounds.html) on their type parameters.

## Syntax

### Defining & Implementing

```rust
pub trait Tweeting {
    fn tweet(&self) -> String;
    // can also have types and constants
}
pub struct Squirrel {
    pub name: String,
    pub legs: i32,
    pub tone: String,
}

// impl Trait
impl Tweeting for Squirrel {
    fn tweet(&self) -> String {
        format!("{}, zhizhizhi", self.tone)
    }
}

// conditionally impl with trait bound syntax
struct Pair<T> {
    x: T,
    y: T,
}
impl<T: Display + Tweeting > Pair<T> {
		...
}
```

Implementations of a trait on any type that satisfies the trait bounds are called ***blanket implementations**.*

### As Function Parameters

```rust
// impl Trait
fn some_function(t: (impl Display + Clone), u: (impl Clone + Debug))-> i32 {
...
}

// trait bound
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
...
}

// using where clause
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
...
}
```

### As Returning Types

```rust
fn returns_something() -> impl SonmeType{
	//snip
}
```

# Advanced

## Generic traits

*Associated types* connect a type **placeholder** with a trait such that the trait method definitions can use these placeholder types in their signatures. 

```rust
pub trait Iterator {
    type Item; // <- the output type in <>

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        
// generic version
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

## Trait Objects

A trait object points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime. We can use trait objects in place of a generic **or** concrete type. 

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

This works differently from defining a struct that uses a generic type parameter with trait bounds. A **generic type parameter can only be substituted with one concrete type at a time**, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime. 

A *trait object* is an opaque value of another type that implements a set of traits. The set of traits is made up of a [dyn compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility) *base trait* plus any number of [auto traits](https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits).
`Dyn` compatibility:

- All [supertraits](https://doc.rust-lang.org/reference/items/traits.html#supertraits) must also be dyn compatible.
- It must not require `Self: Sized`.
- It must not have any associated **constants**.
- It must not have any associated types with **generics**.
- All associated functions must either be dispatchable from a trait object or be explicitly non-dispatchable.

The [`AsyncFn`](https://doc.rust-lang.org/core/ops/async_function/trait.AsyncFn.html), [`AsyncFnMut`](https://doc.rust-lang.org/core/ops/async_function/trait.AsyncFnMut.html), and [`AsyncFnOnce`](https://doc.rust-lang.org/core/ops/async_function/trait.AsyncFnOnce.html) traits are not dyn-compatible.

```rust
trait Printable {
    fn stringify(&self) -> String;
}

impl Printable for i32 {
    fn stringify(&self) -> String { self.to_string() }
}

fn print(a: Box<dyn Printable>) {
    println!("{}", a.stringify());
}

fn main() {
    print(Box::new(10) as Box<dyn Printable>);
}
```

In this example, the trait `Printable` occurs as a trait object in both the type signature of `print`, and the cast expression in `main`.

Since a trait object can contain references, the lifetimes of those references need to be expressed as part of the trait object. This lifetime is written as `Trait + 'a`.

## Default Generic Type Parameters

You can specify a default type when declaring a generic type with the `<PlaceholderType=ConcreteType>` syntax.

You’ll use default type parameters in two main ways:

- To extend a type without breaking existing code
- To allow customization in specific cases most users won’t need

```rust
// rhs = “right hand side”
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

## Disambiguation: Calling Methods with the Same Name

When calling **methods** with the same name, you’ll need to tell Rust which one you want to use.

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

For **associated functions** that aren’t methods (without `self` parameter), there would not be a `receiver`: there would only be the list of other arguments. You could use fully qualified syntax everywhere that you call functions or methods. However, you’re allowed to omit any part of this syntax that Rust can figure out from other information in the program.

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

## Supertraits

For a type to implement the first trait, you want to require that type to **also implement the second trait**. You would do this so that your trait definition can make use of the associated items of the second trait. **The trait your trait definition is relying on is called a *supertrait*** of your trait.

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

## Newtype Pattern

**orphan rule**: We’re only allowed to implement a trait on a type if either the trait or the type are **local** to our crate. It’s possible to get around this restriction using the ***newtype pattern***, which involves creating a new type in a tuple struct.

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
```

The implementation of `Display` uses `self.0` to access the inner `Vec<T>`, because `Wrapper` is a tuple struct and `Vec<T>` is the item at index 0 in the tuple. Then we can use the functionality of the `Display` trait on `Wrapper`.

# Reference

[Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html)

[Advanced Traits - The Rust Programming Language](https://doc.rust-lang.org/beta/book/ch20-02-advanced-traits.html)

[Traits - The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html)

[Traits - Rust By Example](https://doc.rust-lang.org/rust-by-example/trait.html)