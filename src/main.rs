use tokio::time::{Duration, sleep};
use rand::random;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    println!("start program");
    afs_borrowed().await;
    afs_not_borrowed().await;
    afs_borrowed_zipped().await;
    println!("finish program");
}

async fn my_async_function(num: i32, str3: &str) {
    println!("started async function {num}");
    let d1 = random::<u64>() % 900 + 100;
    let d = Duration::from_millis(d1);
    sleep(d).await;
    println!("finished async function {num} |{str3}| slept {d:?}");
} 

async fn afs_not_borrowed() {
    let str1 = "somehow not borrowed string";
    let vs = (0..10).collect::<Vec<i32>>();
    let mut hs = vec!();
    for v in vs {
        let h = tokio::spawn({
            async move {
                my_async_function(v, &str1).await;
            }
        });
        hs.push(h);
    }

    for h in hs {
        h.await.unwrap();
    }
}

async fn afs_borrowed() {
    let str1 = "borrowed string (Arc hell)".to_string();
    let str_arc1 = Arc::new(str1);

    let vs = (0..10).collect::<Vec<i32>>();
    let mut hs = vec!();
    for v in vs {
        let h = tokio::spawn({
            let str_arc2 = Arc::clone(&str_arc1);
            async move {
                async {
                    let x = str_arc2.as_str();
                    my_async_function(v, x).await 
                }.await
            }
        });
        hs.push(h);
    }

    for h in hs {
        h.await.unwrap();
    }
}


/*
Looks smart but has the costs one copy of all data for each thread.
Evtl. someone could argue, creating the copies never takes place ???
 */
async fn afs_borrowed_zipped() {
    let str1 = "borrowed string (zipped)".to_string();
    let nums = (0..10).collect::<Vec<i32>>();
    let strs = vec![str1; nums.len()];
    let vs = nums
        .iter()
        .zip(strs)
        .map(|(a, b)| (*a, b))
        .collect::<Vec<_>>();

    let mut hs = vec!();
    for (num, str2) in vs {
        let h = tokio::spawn({
            async move {
                my_async_function(num, &str2).await;
            }
        });
        hs.push(h);
    }

    for h in hs {
        h.await.unwrap();
    }
}
