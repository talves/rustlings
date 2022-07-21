// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: Mutex<u32>,
    not_mutex: String,
}

fn main() {
    let status = Arc::new(JobStatus {
        jobs_completed: Mutex::new(0),
        not_mutex: String::from("This value is not changed"),
    });
    let mut handles = vec![];
    for i in 0..20 {
        let status_shared = status.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut count = status_shared.jobs_completed.lock().unwrap();
            // status_shared.jobs_completed += 1;
            println!("thread {} is complete (count:{})", i, *count + 1);
            *count += 1
        });

        handles.push(handle);
    }

    // Move this println around to see if it waits for the joins
    println!("value of not_mutex: {}", status.not_mutex);

    for handle in handles {
        handle.join().unwrap();
    }
    // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
    // interesting in the output? Do you have to 'join' on all the handles?
    // YES: If you comment out the for in above, you will see there isn't a wait for the join
    println!("jobs completed {:?}", status.jobs_completed.lock().unwrap());
}
