use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

async fn person ( real_arch : Arc<Mutex<i32>>, name : String, new_channel : i32 )
{
    let mut real_remote = real_arch.lock().await;
    *real_remote = new_channel;

    println!( "Remote is with {} ", name );

    println! ("Watching channel {} ", new_channel) ;

    sleep (Duration::from_secs (5)).await;

}
#[tokio::main]
async fn main() {

    let tv_channel = 10;
    let  mutex = Mutex::new(tv_channel);
    let real_arch = Arc::new(mutex);

    let mut task_handles = Vec::new();

    for ( name, new_channel) in [("x", 11 ), ("y", 12), ("z", 13)]
    {
        task_handles.push ( person (real_arch.clone(), name.to_string(), new_channel.clone()));
    }

    for  handle in task_handles 
    {
        handle.await;
    }
}
