use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

async fn read_from_document(id: i32, document: Arc<RwLock<String>>) {
    let reader = document.read().await;
    println!("reader_{}: {}", id, *reader);
}

async fn write_to_document(new_string: &str, document: Arc<RwLock<String>>) {
    let mut writer = document.write().await;
    writer.push_str(new_string);
    writer.push_str(" ");
}

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();

    let document = Arc::new(RwLock::new("".to_string()));
    for new_string in "I am a new rust programmer of e m b i t e l".split_whitespace() {
        handles.push(tokio::spawn(read_from_document(1, document.clone())));
        handles.push(tokio::spawn(write_to_document(
            new_string,
            document.clone(),
        )));
        sleep(Duration::from_millis(1)).await;
        handles.push(tokio::spawn(read_from_document(2, document.clone())));
        handles.push(tokio::spawn(read_from_document(3, document.clone())));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
