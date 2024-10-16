use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{Duration, sleep};

async fn person ( semaphore : Arc <Semaphore<> >, name : String )
{
    println!("Waiting for the teller {} ", name );
    teller ( semaphore , name ).await;

}

async fn teller ( semaphore : Arc <Semaphore<>> , name : String)
{
   let permit = semaphore.acquire().await;
   sleep ( Duration::from_secs (2)).await;
   println!("{} is being served ", name );
   sleep (Duration::from_secs(5)).await;
   println!("{} is now leaving the teller  ", name );
   drop (permit);

}

#[tokio::main]
async fn main() {

    let num_of_tellers = 4;
    let semaphore = Semaphore::new(num_of_tellers);
    let arc_sem = Arc::new (semaphore);

    let mut people_handles = Vec::new();

    for num in 0..10 {
        people_handles.push ( tokio::spawn (person(arc_sem.clone(), format!("Person_{num}"))));
    }

    for handle in people_handles 
    {
        handle.await.unwrap();
    }

}
