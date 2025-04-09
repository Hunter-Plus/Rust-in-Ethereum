# Concepts of Concurrent programming

## Concurrency and Parallelism

Concurrency is about ordering of computations and parallelism is about the mode of execution.

Concurrency is a way of organizing code and parallelism is a resource. That concurrency is about organizing code rather than executing code is important because from the perspective of the processor, concurrency without parallelism simply doesn't exist.

- Concurrency is about ordering of computation (operations are concurrent if their order of execution cannot be observed).
- Parallelism is about computing on multiple processors (operations are parallel if they are literally happening at the same time).

## Processes and Threads

The most important difference is that memory is shared between threads but not between processes. From a program's perspective, the single process is their whole world; creating new processes means running new programs. Creating new threads, however, is just part of the program's regular execution.

When multiple threads are run on a single core in this fashion, it is called ***interleaving*** or ***time-slicing***. Since the OS chooses when to pause a thread's execution, it is called ***pre-emptive multitasking**.* When an OS pauses one thread and starts another (for any reason), it is called ***context switching***.

## Async Programming

The two big differences between async concurrency and concurrency with threads, is that async concurrency is managed entirely within the program **with no help from the OS**, and that multitasking is cooperative rather than pre-emptive. 

Asynchronous programming is managed by a user-space runtime. Multi-tasking is cooperative. It has lower overheads than threads.

# Features in Rust Async Programming

## `Future`trait

[Future in std::future - Rust](https://doc.rust-lang.org/nightly/std/future/trait.Future.html)

A future represents an asynchronous computation task(async task). In Rust, a future is an object which implements the [`Future`](https://doc.rust-lang.org/nightly/std/future/trait.Future.html) trait. Rust do not have a concept of **task** while most runtime implementations have this concept.

```rust
pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

To use a future, one just has to repeatedly call the `poll` method until it returns:

```rust
enum Poll<T> {
    Ready(T),
    Pending,
}
```

- [`Poll::Pending`](https://doc.rust-lang.org/nightly/std/task/enum.Poll.html#variant.Pending) if the future is not ready yet
- [`Poll::Ready(val)`](https://doc.rust-lang.org/nightly/std/task/enum.Poll.html#variant.Ready) with the result `val` of this future if it finished successfully.

Currently, `Context` only serves to provide access to a `&Waker` which can be used to wake the current **task**. It also stores the data will be used when the task is ready to execute.

## Executors

[futures::executor - Rust](https://docs.rs/futures/latest/futures/executor/index.html)

We use `futures` crate as example which is just one way to implement.

All asynchronous computation occurs within an executor, which is capable of spawning futures as tasks. The key job for the executors are calls to the `poll` method. We cannot do it too often neither too infrequently. A good executor must interact with the OS so that it can wake futures at the optimal time (this is sometimes called a reactor).

Executors typically present two key APIs: `block_on` and `spawn`.

```rust
pub fn block_on<F>(f: F) -> <F as Future>::Output
where
    F: Future,
```

`block_on` will block the caller until the given future has completed. (Blocking any other activity in the current thread, and returning when the future is done.)

`spawn` has many implementation in crates, generally, it spawns a task that polls the given future with various types of outputs as returns. (runs a future without blocking the current thread, returning immediately.)

## runtime

The essential part of an async runtime is the executor, but an async runtime also includes libraries which make it practical to write async code. As well as running and scheduling tasks, a runtime must interact with the OS to manage async IO.

- *reactor* or *event loop* or *driver* : dispatches IO and timer events, interacts with the OS, and does the lowest-level driving forward of execution,
- *scheduler*: determines when tasks can execute and on which OS threads,
- *executor* or *runtime*: combines the reactor and scheduler, and is the user-facing API for running async tasks; *runtime* is also used to mean the whole library of functionality .

Rust is a low-level language and strives towards minimal runtime overhead. The async runtime therefore has a much more limited scope than many other languages' runtimes. There are also many ways to design and implement an async runtime, so Rust lets you choose one depending on your requirements, rather than providing one. 

### `task`, `spawn`, and runtime

Using [`tokio::spawn`](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html) as an example: The `spawn` function takes a future and runs it as a new Tokio task. Tasks are the the concept which the Tokio runtime schedules and manages. Tokio is a multi-threaded runtime which means that when we spawn a new task, that task may be run on a different OS thread from the task it was spawned from.

So, when a future is spawned as a task it runs *concurrently* with the task it was spawned from and any other tasks. It may also run in parallel to those tasks if it is scheduled on a different thread.

If we use either `thread::spawn` or `tokio::spawn` we introduce concurrency and potentially parallelism, in the first case between threads and in the second between tasks.

### Joining tasks

`spawn` is not an `async` function, it's a regular function that returns a future (`JoinHandle`).

When a task is spawned, the spawn function returns a [`JoinHandle`](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html) , if you want the spawning task to wait for the spawned task to complete and then use the result, you can `await` the `JoinHandle` to do so. `JoinHandle` is a generic type and it's type parameter is the type returned by the spawned task. 

### **Futures**

The [futures](https://github.com/rust-lang/futures-rs) crate is a Rust-official crate. It started as a place to develop the `Future` trait and associated libraries, on their way to inclusion in Rust's standard library.
It includes a basic executor and a bunch of useful traits for streams, IO, etc. However, it omits the low-level functionality for async IO, so it is not a complete solution.

futures in Rust are *lazy*: they don’t do anything until you ask them to with the `await` keyword. 

### **Tokio**

[Tokio](https://tokio.rs/) is one of the oldest and mostly widely-used runtimes, and probably the most used in production. Tokio's IO traits are a bit different from most others in the ecosystem and the synchronous versions in the standard library. Specifically, data is read into an abstract buffer type, rather than an `&mut [u8]`. That makes them more flexible and sometimes more performant, but also a little trickier to use.

### **Smol and async-std**

[Async-std](https://async.rs/) and [Smol](https://github.com/smol-rs/smol) are two runtimes based on smol-rs components. Async-std is designed to be as close to the synchronous standard library as possible, while Smol is designed to be more minimal.

### **Glommio**

[Glommio](https://github.com/DataDog/glommio) is a specialised runtime based on the thread-per-core philosophy and implemented using io_uring. It is primarily designed for disk IO.

### **Embassy**

[Embassy](https://github.com/embassy-rs/embassy) is a runtime designed specifically for embedded development. In particular, it avoids allocation and does not require a heap.

### **Bastion**

[Bastion](https://bastion.rs/) is an actors runtime, and thus a bit higher-level and more opinionated than general purpose runtimes. It is designed to be fault-tolerant and highly available. It doesn't have libraries for general IO.

## `async` keyword

The `async` keyword is a modifier on function declarations. An async function is simply a function declared using the `async` keyword, and what that means is that it is a function which can be executed asynchronously, in other words the caller *can choose not to* wait for the function to complete before doing something else.

When an async function is called, the body is not executed as it would be for a regular function. Instead the function body and its arguments are **packaged into a future** which is returned in lieu of a real result.

An async block is the simplest way to start an async context and create a future. It is commonly used to create small futures which are only used in one place. An async block scopes code and names together, but at runtime it is not immediately executed and evaluates to a future. 

Writing `async fn` is equivalent to writing a function that returns a *future* of the return type. `async main fn` is not allowed.

## `await` keyword

If the result is ready immediately or can be computed without waiting, then `await` simply does that future computation to produce the result. However, if the result is not ready, then `await` hands control over to the scheduler so that another task can proceed.

`await` is an operator which continues execution of the current task, or if the current task can't continue right now, gives another task an opportunity to continue. `async` functions are one way to define a future, and `await` is one way to combine futures. Using `await` on a future combines that future into the future produced by the async function it's used inside. 

## The `Pin` and `Unpin` Traits

`Pin` is a wrapper for pointer-like types such as `&`, `&mut`, `Box`, and `Rc`. (Technically, `Pin` works with types that implement the `Deref` or `DerefMut` traits). When we *pin* a value by wrapping a pointer to that value in `Pin`, it can no longer move. 

`Unpin` is a marker trait which has no functionality of its own. `Unpin` informs the compiler that a given type **does *not* need to uphold any guarantees** about whether the value in question can be safely moved. Items in containers that can safely be moved(like `Vec`) or behind pointers like `Box` , are all safe to be moved. We use `Unpin`to tell the compiler that it’s fine to move items around in cases like this.
`!Unpin`is the special case. Just as with `Send` and `Sync`, the compiler implements `Unpin` automatically for all types where it can prove it is safe. The notation for `!Unpin` is `impl !Unpin for *SomeType*`, where `*SomeType*` is the name of a type that ***does*** need to uphold those guarantees to be safe whenever a pointer to that type is used in a `Pin`.
Whether a type implements `Unpin` or `!Unpin` *only* matters when you’re using a pinned pointer to that type like `Pin<&mut *SomeType*>`.

### Moving a future

**Await Points**: future get compiled into a **state machine**, and the compiler makes sure that state machine follows all of Rust’s normal rules around safety, including borrowing and ownership. To make that work, Rust looks at what data is needed between one await point and either the next await point or the end of the async block. 

When we move a future—whether by pushing it into a data structure to use as an iterator with `join_all` or by returning it from a function—that actually means moving the state machine Rust creates for us.

**Internal Reference**: unlike most other types in Rust, the futures Rust creates for async blocks can end up with references to themselves.

![image.png](attachment:7df83aa6-3ec4-43c2-ad83-d31689aca109:image.png)

Any object that has a reference to itself is unsafe to move, because references always point to the actual memory address of whatever they refer to. If you move the data structure itself, those internal references will be left pointing to the old location.

## `Stream` Trait, the Future Iterator

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

The outer type is `Poll`, because it has to be checked for readiness, just as a future does. The inner type is `Option`, because it needs to signal whether there are more messages, just as an iterator does.

```rust
trait StreamExt: Stream {
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;

    // other methods...
}
```

The `StreamExt` trait supplies the `next` method. `StreamExt` is automatically implemented for every type that implements `Stream`, but these traits are defined separately to enable the community to iterate on convenience APIs without affecting the foundational trait.

# Topics when using async

## Task Racing

Which task will be executed first when there are multiple async tasks awaiting.

At each await point, Rust gives a runtime a chance to pause the task and switch to another one if the future being awaited isn’t ready. Rust *only* pauses async blocks and hands control back to a runtime at an await point. If you do a bunch of work in an async block without an await point, that future will block any other futures from making progress. You may sometimes hear this referred to as one future ***starving*** other futures.

## async with concurrence

async can be used with concurrence. For example, all codes within a async block will be executed sequentially, and all futures in one block will race for execution.

With threads, we used the `join` method to “block” until the thread was done running.
When you give it two futures, it produces a single new future whose output is a tuple containing the output of each future you passed in once they *both* complete.
With threads, the operating system decides which thread to check and how long to let it run. With async Rust, the runtime decides which task to check.

The general-purpose stream API is much broader: it provides the next item the way `Iterator` does, but asynchronously.
`Stream` trait defines a low-level interface that effectively combines the `Iterator` and `Future` traits. 

`StreamExt` supplies a higher-level set of APIs on top of `Stream`, including the `next` method as well as other utility methods similar to those provided by the `Iterator` trait.
`Stream` and `StreamExt` are not yet part of Rust’s standard library.

### Composing Streams

channel is *unbounded*: it can hold as many messages as we can fit in memory. 

### Merging Streams

with the `merge` method, which combines multiple streams into one stream that produces items from any of the source streams as soon as the items are available, without imposing any particular ordering.

## Messaging between futures

Messgaing between `futures` uses a mutable rather than an immutable receiver `rx`, and its `recv` method produces a future we need to await rather than producing the value directly.
The synchronous `Receiver::recv` method in `std::mpsc::channel` blocks until it receives a message. The `trpl::Receiver::recv` method does not, because it is async. Instead of blocking, it hands control back to the runtime until either a message is received or the send side of the channel closes.

If we want some variants being dropped at the end of the async block, we should use `move` combining with async.

## Yielding control

using `std:::task::yield_now` to give back the control

*cooperative multitasking*, where each future has the power to determine when it hands over control via await points. Each future therefore also has the responsibility to avoid blocking for too long. In some Rust-based embedded operating systems, this is the *only* kind of multitasking!

## async abstraction

See the code examples of `async fn timeout<F: Future>`.