use tokio::time::{Duration, sleep};
use rand::random;
use futures::TryStreamExt;
use futures::stream;

#[derive(Debug)]
struct ForcedTestErr;

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

#[tokio::main]
async fn main() {
    println!("---> main");
    with_stream().await;
    println!("<--- main");
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
    println!("---> with stream");
    let datas = &create_datas();
    let result_nums = datas
        .iter()
        .map(|x| Ok(x))
        .collect::<Vec<Result<_, _>>>();
    let result_all = stream::iter(result_nums)
        .try_for_each_concurrent(
            Some(3),
            |value| async move { my_async_function(&value).await },
        ).await;
    match result_all {
        Ok(_) => println!("# concurrent execution of {} data was successful", datas.len()),
        Err(e) => println!("# concurrent execution resulted in Error: {:?}", e),
    }
    println!("<--- with stream");
}

async fn my_async_function(data: &MyData) -> Result<(), ForcedTestErr> {
    println!("---> async function result {} {} {:?}", data.id, data.name, data.lang);
    let ran_num = random::<u64>() % 1000;
    if ran_num < 100 {
        return Err(ForcedTestErr);
    }
    let dur = Duration::from_millis(ran_num + 10);
    sleep(dur).await;
    println!("<--- async function result {} slept {:?}", data.id, dur);
    Ok(())
}

