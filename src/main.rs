
#[tokio::main]
async fn main() {
    println!("start program");
    af().await;
    println!("finish program");
}

async fn af() {
    println!("executing inside async");
} 
