use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
        // The thread can return a value
        42
    });

    // Code in the main thread runs concurrently
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the spawned thread to finish and get its result
    let result = handle.join().unwrap();
    println!("Spawned thread returned: {result}");
}
