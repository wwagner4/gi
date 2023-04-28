use tokio::time::{Duration, sleep};
use rand::random;


#[tokio::main]
async fn main() {
    println!("start program");
    let vs = (0..10).collect::<Vec<i32>>();
    for v in vs {
        af(v).await;
    }
    println!("finish program");
}

async fn af(num: i32) {
    let d1 = random::<u64>() % 900 + 100;
    let d = Duration::from_millis(d1);
    sleep(d).await;
    println!("executing async function {num} slept {d:?}");
} 
