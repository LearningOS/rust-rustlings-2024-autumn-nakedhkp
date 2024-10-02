use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // Wrap JobStatus in both Arc and Mutex
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // Lock the Mutex before updating the shared value
            let mut num = status_shared.lock().unwrap();
            num.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // Join all the threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of jobs_completed
    println!("jobs completed {}", status.lock().unwrap().jobs_completed);
}
