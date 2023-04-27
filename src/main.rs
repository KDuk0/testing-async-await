use tokio::spawn;
use tokio::time::{delay_for, Duration};

#[tokio::main]
async fn main() {
    let async_task = spawn(async {
        println!("Async task started.");
        delay_for(Duration::from_secs(1)).await;
        println!("Async task done.");
    });

    println!("Launching task...");
    async_task.await.expect("asyinc task failed");
    println!("ready!");
}