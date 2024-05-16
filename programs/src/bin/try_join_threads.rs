use std::thread;
use std::time::Duration;
use tokio::time::sleep;
use tokio::try_join;


async fn task1() {
    // Simulating some asynchronous work
    sleep(Duration::from_secs(2)).await;
    println!("Task 1 completed");
}

async fn task2() {
    // Simulating some asynchronous work
    sleep(Duration::from_secs(3)).await;
    println!("Task 2 completed");
}

async fn task3() {
    // Simulating some asynchronous work
    sleep(Duration::from_secs(4)).await;
    println!("Task 3 completed");
}

fn task4() -> Result<(), ()> {
    // Simulating some asynchronous work
    sleep(Duration::from_secs(5));
    println!("Task 4 completed");
    Ok(())
}

// Wrapper around a synchronous task to convert it into a Future
async fn sync_task_future<T, F>(task: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(task).await.unwrap()
}
// Wrapper around JoinHandle to convert it into a Future
async fn join_handle_future<T>(handle: thread::JoinHandle<T>) -> T {
    handle.join().expect("Failed to join thread")
}

#[tokio::main]
async fn main() {
    // Spawning two threads for executing tasks
    let handle1 = tokio::spawn(task1());
    let handle2 = tokio::spawn(task2());
    let handle3 = tokio::spawn(task3());
    let handle4 = tokio::spawn(sync_task_future(task4));

    // Using `try_join` to wait for both tasks to complete
    let result = try_join!(
        handle1,
        handle2,
        handle3,
        handle4,
    )
    .map(|_| {
        println!("All tasks completed successfully");
    });

    // Handling errors, if any
    if let Err(e) = result {
        eprintln!("Error: {:?}", e);
    }
}
