use tokio::spawn;
use tokio::time::{delay_for, Duration};

#[tokio::main]
async fn main() {
    let mut tasks = vec![];
    for id in 0..5 {
        let t = spawn(async move {
            println!("Async task {} started.", id);
            delay_for (Duration::from_millis((5-id) * 100)).await;
            println!("Async task {} done.", id);
            let result = id * id;
            (id, result)
        });

        tasks.push(t);
    }
    
    println!("Launched {} tasks...", tasks.len());
    for task in tasks {
        let (id, result) = task.await.expect("task failed");
        println!("Task {} completed with result: {}", id, result);
    }

    println!("Ready");
    
}