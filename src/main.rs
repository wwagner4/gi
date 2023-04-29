use tokio::time::{Duration, sleep};
use rand::random;
use futures::{StreamExt};
use futures::stream;


#[tokio::main]
async fn main() {
    println!("-> main");
    with_stream().await;
    println!("<- main");
}

struct MyData {
    id: u8,
    name: String,
    lang: Vec<String>,
}

impl MyData {
    fn create(id: u8, name: &str, lang: &Vec<String>) -> Self {
        MyData { id: id, name: name.to_string(), lang: lang.clone() }
    }
}

fn create_datas() -> Vec<MyData> {
    let lang = vec!("DE".to_string(), "FR".to_string(), "ES".to_string());
    vec!(
        MyData::create(0, "erwin", &lang),
        MyData::create(1, "fromm", &lang),
        MyData::create(2, "leben", &lang),
        MyData::create(3, "liebe", &lang),
        MyData::create(4, "lebendig", &lang),
    )
}

async fn with_stream() {
    println!("--> with stream");
    let datas = create_datas();
    stream::iter(datas)
        .for_each_concurrent(
            Some(30), |value| async move {
                my_async_function(&value).await
            },
        ).await;
    println!("<-- with stream");
}

async fn my_async_function(data: &MyData) -> () {
    println!("---> async function {} |{}| {:?}", data.id, data.name, data.lang);
    let d1 = random::<u64>() % 1000 + 10;
    let d = Duration::from_millis(d1);
    sleep(d).await;
    println!("<--- async function {} |{}| {:?} slept {:?}", data.id, data.name, data.lang, d);
}

