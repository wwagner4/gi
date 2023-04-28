use tokio::time::*;


#[tokio::main]
async fn main() {
    println!("start program");
    af(23).await;
    af(24).await;
    af(25).await;
    af(26).await;
    println!("finish program");
}

async fn af(num: u32) {
    let d = Duration::from_millis(1000);
    sleep(d);
    println!("executing async function {num} slept {d:?}ms");
} 
