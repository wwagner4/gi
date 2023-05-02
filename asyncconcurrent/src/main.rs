use tokio::time::{Duration, sleep};
use rand::random;
use futures::TryStreamExt;
use futures::stream;

#[derive(Debug)]
struct ForcedTestErr;

struct MyData {
    id: u8,
    info: String,
    lang: Vec<String>,
}

impl MyData {
    fn create(id: u8, name: &str, lang: &Vec<String>) -> Self {
        MyData { id: id, info: name.to_string(), lang: lang.clone() }
    }
}

#[tokio::main]
async fn main() {
    println!("---> main");
    with_stream().await;
    println!("<--- main");
}

fn create_datas() -> Vec<MyData> {
    let ids = 0..10;
    let lang = vec!("DE".to_string(), "FR".to_string(), "ES".to_string());
    let info = "Some Info";
    ids.into_iter()
        .map(|i|MyData::create(i, info, &lang))
        .collect::<Vec<_>>()
}

async fn with_stream() {
    println!("---> with stream");
    let datas = &create_datas();
    let result_nums = datas
        .iter()
        .map(|x| Ok(x))
        .collect::<Vec<Result<_, _>>>();
    let result_all = stream::iter(result_nums)
        .try_for_each_concurrent(
            None,
            |value| my_async_function(&value),
        ).await;
    match result_all {
        Ok(_) => println!("# concurrent execution of {} data was successful", datas.len()),
        Err(e) => println!("# concurrent execution resulted in Error: {:?}", e),
    }
    println!("<--- with stream");
}

async fn my_async_function(data: &MyData) -> Result<(), ForcedTestErr> {
    println!("---> async function result {} {} {:?}", data.id, data.info, data.lang);
    let ran_num = random::<u64>() % 5000;
    if ran_num < 100 {
        return Err(ForcedTestErr);
    }
    let dur = Duration::from_millis(ran_num + 10);
    sleep(dur).await;
    println!("<--- async function result {} slept {:?}", data.id, dur);
    Ok(())
}

