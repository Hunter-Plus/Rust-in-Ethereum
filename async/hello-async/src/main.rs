use std::{pin::Pin, pin::pin, thread, time::Duration};
use trpl::{Either, Html, ReceiverStream, Stream, StreamExt};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // basic futures and task race
    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    });
    // async with concurrence

    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..5 {
                println!("hi number {i} from the first JoinHandle task!");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        for i in 1..10 {
            println!("hi number {i} from the second JoinHandle task!");
            trpl::sleep(Duration::from_millis(100)).await;
        }

        handle.await.unwrap();
    });

    trpl::run(async {
        let future1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first Future task!");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        let future2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second Future task!");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        // handle is a join handle which will block the main process from stopping
        trpl::join(future1, future2).await;
    });

    // messaging between futures
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
    });

    // sending and receiving are still in sequence
    // trpl::run(async {
    //     let (tx, mut rx) = trpl::channel();

    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("future"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         trpl::sleep(Duration::from_millis(100)).await;
    //     }

    //     while let Some(value) = rx.recv().await {
    //         println!("received '{value}'");
    //     }
    // });

    // using join to make then concurrent
    trpl::run(async {
        // move the tx(Sender) so it will be drpped at the end of the async block
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };
        // we need unique functions for each number of futures, not realistic
        // trpl::join3(tx1_fut, tx_fut, rx_fut).await;

        // we still need to know the number of futures before complie time, not realistic
        // however, by using macros, we can ignore the Output type and combine them together easily
        // trpl::join!(tx1_fut, tx_fut, rx_fut);

        // using trait objects to build the vector
        // use future: Vec<Box<dyn Future<Output = ()>>> rather Vec<Box<impl Future<Output = ()>>>
        // add Pin
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
        trpl::join_all(futures).await;
    });

    // racing futures
    trpl::run(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
    });

    // yield back the control to runtime, make sure all lines executed
    trpl::run(async {
        let a = async {
            println!("'A' started.");
            slow("A", 30);
            trpl::yield_now().await;
            slow("A", 10);
            trpl::yield_now().await;
            slow("A", 20);
            trpl::yield_now().await;
            println!("'A' finished.");
            trpl::yield_now().await;
        };

        let b = async {
            println!("'B' started.");
            slow("B", 75);
            trpl::yield_now().await;
            slow("B", 10);
            trpl::yield_now().await;
            slow("B", 15);
            trpl::yield_now().await;
            slow("B", 35);
            // trpl::yield_now().await;
            println!("'B' finished.");
            trpl::yield_now().await;
        };
        trpl::race(a, b).await;
    });

    // Using my own async abstraction
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });

    // streaming futures
    // StreamExt provides the next() method
    trpl::run(async {
        let values = 1..11;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });

    // a demo for streaming messages
    trpl::run(async {
        let mut messages = get_messages();

        while let Some(message) = messages.next().await {
            println!("Streaming message received: {message}");
        }
    });

    // streaming messages with timeout
    trpl::run(async {
        let mut messages =
            //the timeout helper produces a stream that needs to be pinned to be polled
            pin!(get_messages_with_delay().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("Streaming message with delays:{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });

    // merging streams
    trpl::run(async {
        let messages = get_messages_with_delay().timeout(Duration::from_millis(200));
        // we must convert u32 to string when merging
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut merged_stream = pin!(merged);
        while let Some(result) = merged_stream.next().await {
            match result {
                Ok(message) => println!("Merged Message:{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

// stream of messages
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }

    ReceiverStream::new(rx)
}

// streaming messages with delays
fn get_messages_with_delay() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

// send a messgae per millisecond
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
