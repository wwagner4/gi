use tokio::time::{Duration, sleep};
use rand::random;
use futures::{StreamExt};
use futures::stream;


#[tokio::main]
async fn main() {
    println!("---> main");
    with_stream().await;
    println!("<--- main");
}

async fn with_stream() {
    println!("---> with stream");
    let nums = (0..15).collect::<Vec<i32>>();
    stream::iter(nums)
        .for_each_concurrent(
            Some(10), |value| async move {
                my_async_function(value).await
            },
        ).await;
    println!("<--- with stream");
}

async fn my_async_function(num: i32) -> () {
    println!("---> async function {num}");
    let d1 = random::<u64>() % 3000 + 10;
    let d = Duration::from_millis(d1);
    sleep(d).await;
    println!("<--- async function {num} slept {d:?}");
}

