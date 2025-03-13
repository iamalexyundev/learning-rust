use std::{pin::Pin, thread, time::Duration};

use trpl::{Either, Html};
// fn main() {
//     let args: Vec<String> = std::env::args().collect();
//     trpl::run(async {
//         let title_fut_1 = page_title(&args[1]);
//         let title_fut_2 = page_title(&args[2]);
//         let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
//             Either::Left(left) => left,
//             Either::Right(right) => right,
//         };
//         println!("{url} returned first");
//         match maybe_title {
//             Some(title) => println!("Its page title is: '{title}'"),
//             None => println!("Its title could not be parsed"),
//         }
//     })
// }
// async fn page_title(url: &str) -> (&str, Option<String>) {
//     let response_text = trpl::get(url).await.text().await;
//     let title = Html::parse(&response_text)
//         .select_first("title")
//         .map(|title_element| title_element.inner_html());
//     (url, title)
// }
// fn main() {
//     trpl::run(async {
//         let fut1 = async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };
//         for i in 1..5 {
//             println!("hi nubmer {i} from the second task");
//             trpl::sleep(Duration::from_millis(500)).await;
//         }
//         fut1.await;
//     });
// }

// fn main() {
//     trpl::run(async {
//         let (tx, mut rx) = trpl::channel();

//         let tx1 = tx.clone();
//         let tx1_fut = async move {
//             let vals = vec![
//                 String::from("hi"),
//                 String::from("from"),
//                 String::from("the"),
//                 String::from("future"),
//             ];

//             for val in vals {
//                 tx1.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let tx_fut = async move {
//             let vals = vec![
//                 String::from("more"),
//                 String::from("messages"),
//                 String::from("for"),
//                 String::from("you"),
//             ];

//             for val in vals {
//                 tx.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let rx_fut = async {
//             while let Some(value) = rx.recv().await {
//                 println!("received '{value}'");
//             }
//         };
//         let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
//             vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
//         trpl::join_all(futures).await;
//     })
// }

fn main() {
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
    })
}
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
