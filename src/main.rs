use tokio::time::{Duration, sleep};
use rand::random;
use futures::TryStreamExt;
use futures::stream;


#[tokio::main]
async fn main() {
    println!("---> main");
    with_stream().await;
    println!("<--- main");
}

async fn with_stream() {
    println!("---> with stream");
    let result_nums  = (0..15)
        .into_iter()
        .map(|x|  Ok(x)  )
        .collect::<Vec<Result<i32, _>>>();
    let result_all = stream::iter(result_nums)
        .try_for_each_concurrent(
            None, |value| async move {
                my_async_function(&value).await
            },
        ).await;
    match result_all {
        Ok(_) => println!("concurrent execution was successful"),
        Err(e) => println!("concurrent execution resulted in Error: {:?}", e),
    }
    println!("<--- with stream");
}

#[derive(Debug)]
struct WantedErr;

async fn my_async_function(num: &i32) -> Result<(), WantedErr>{
    println!("---> async function result {}", num);
    let d1 = random::<u64>() % 1000 + 10;
    if d1 < 100 {
        return Err(WantedErr);
    }
    let d = Duration::from_millis(d1);
    sleep(d).await;
    println!("<--- async function result {} slept {:?}", num, d);
    Ok(())
}
