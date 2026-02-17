use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

fn main (){
    // 1. Wrap your vector in a Mutex, then in an Arc "Arc<Mutex<Vec<i32>>> "
    let data: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![1,2,3]));

    // 2. We need a "handle" to the data for the new thread. We .clone() the Arc (this is cheap, it just bumps a counter)
    let data_clone: Arc<Mutex<Vec<i32>>> = Arc::clone(&data);

    // 3. Spawn the new thread, giving it the cloned Arc
    let handle: JoinHandle<()> = thread::spawn(move || {
        // 4. .lock() the mutex to get access to the data. The .unwrap() is used here for simplicity. This helps to lock blocks until the main thread releases it. 
        let mut data: MutexGuard<Vec<i32>> = data_clone.lock().unwrap();
        data.push(4);
        println!("Thread 1 sees: {:?}", data);
        // Mutex is automatically "unlocked" when data goes out of scope
    });

    // 5. Do the "same thing" on the main thread. Afterward use .lock() the original Arc's Mutex.
    let mut data: MutexGuard<'_, Vec<i32>> = data.lock().unwrap();
    data.push(5);
    println!("Main thread sees: {:?}", data);
    // Mutex is unlocked here

    // 6. Wait for the spawned thread to finish
    handle.join().unwrap();
}