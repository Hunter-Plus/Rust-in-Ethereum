use std::time::Duration;
use trpl::{Either, Html};

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
        let (tx, mut rx) = trpl::channel();
        let tx_fut = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!(" async received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;

    });
    

}
