use tokio::time::{Duration, sleep};
use rand::random;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    println!("start program");
    let str2 = "hallo...".to_string();
    afs(&str2).await;
    println!("finish program");
}

async fn afs(str2: &str) {
    let vs = (0..10).collect::<Vec<i32>>();
    let mut hs = vec![];
    let str2_arc = Arc::new(str2.to_owned());
    for v in vs {
        let h = tokio::spawn({
            let str2_arc = Arc::clone(&str2_arc);
            async move {
                af(v, &str2_arc).await;
            }
        });
        hs.push(h);
    }

    for h in hs {
        h.await.unwrap();
    }
}

async fn af(num: i32, str3: &Arc<String>) {
    println!("started async function {num}");
    let d1 = random::<u64>() % 900 + 100;
    let d = Duration::from_millis(d1);
    sleep(d).await;
    let s3 = str3.as_str();
    println!("finished async function {num} |{s3}| slept {d:?}");
} 
