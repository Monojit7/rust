use std::sync::Arc;
use tokio::sync::Notify;
use tokio::time::{sleep, Duration};

async fn package_deliver(package_deliver_arc: Arc<Notify>) {
    println!("Find package ");
    sleep(Duration::from_secs(5)).await;
    println!("Delivering package ");

    sleep(Duration::from_secs(3)).await;
    println!("package deliveried ");

    package_deliver_arc.notify_waiters();
}

async fn grab_package(package_deliver_arc: Arc<Notify>) {
    package_deliver_arc.notified().await;

    println!("package delivery completed ");
}

#[tokio::main]
async fn main() {
    let package_notify = Notify::new();
    let package_notify_arc = Arc::new(package_notify);

    let package_deliver_handle = tokio::spawn(package_deliver(package_notify_arc.clone()));

    let grab_package_handle = tokio::spawn(grab_package(package_notify_arc.clone()));

    package_deliver_handle.await.unwrap();

    grab_package_handle.await.unwrap();
}
