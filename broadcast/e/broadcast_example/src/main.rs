use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();
    let mut handles = Vec::new();
    handles.push(tokio::spawn(async move {
        println!("got message rx1 {} ", rx1.recv().await.unwrap());
    }));

    handles.push(tokio::spawn(async move {
        println!("got message rx2 {} ", rx2.recv().await.unwrap());
    }));

    tx.send(10).unwrap();
    tx.send(20).unwrap();

    for handle in handles {
        handle.await.unwrap();
    }
}
