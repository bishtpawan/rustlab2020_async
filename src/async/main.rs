use async_std::task;
use futures::executor::block_on;
use std::time::Duration;

async fn func_1() {
    println!("func_1 started...");
    // sleeps for 3 second
    task::sleep(Duration::from_secs(3)).await;
    println!("...func_1 end");
}

async fn func_2() {
    println!("func_2 started...");
    // sleeps for 2 second
    task::sleep(Duration::from_secs(2)).await;
    println!("...func_2 end");
}

async fn func_3() {
    println!("func_3 started...");
    // sleeps for 1 second
    task::sleep(Duration::from_secs(1)).await;
    println!("...func_3 end");
}

async fn func_4() {
    println!("func_4 started...");
    println!("...func_4 end");
}

async fn compute() {
    // polls multiple futures simultaneously, returning a tuple of all results once complete.
    futures::join!(func_1(), func_2(), func_3(), func_4());
    println!("Execution successfully completed.");

}

fn main() {
    // run a future to completion on the current thread.
    block_on(compute());
}
