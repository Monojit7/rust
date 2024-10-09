use std::{thread, time};

use tokio::time::{sleep, Duration};

fn blocking_call() -> String {
    thread::sleep(time::Duration::from_secs(5));

    "Finally done".to_string()
}

fn async_call(id: i32) {
    sleep(Duration::from_secs(1));
    println!("Async Call: ID {}", id);
}

#[tokio::main]
async fn main() {
    let blocking_code_handle = tokio::task::spawn_blocking(blocking_call);
        let result = blocking_code_handle.await.unwrap();
    println!("Blocking call: {}", result);
    let mut async_handles = Vec::new();
    for id in 0..10 {
        async_handles.push(tokio::task::spawn_blocking( move || {
            async_call(id)
    }));
    }

     println!("All the async spawn is completed ");
    for handle in async_handles {
        handle.await.unwrap();
    }


}
