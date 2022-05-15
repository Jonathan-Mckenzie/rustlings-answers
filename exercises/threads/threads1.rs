// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // solution: Arc now has a Mutex of JobStatus
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));

    // do we really need a clone?
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(30)); // Reduced from 250ms, humans shouldn't have to wait that long
            let mut job_status = status_shared.lock().unwrap();
            job_status.jobs_completed += 1;
        }
    });
    while status.lock().unwrap().jobs_completed < 10 {
        println!("[main thread] waiting... ");
        thread::sleep(Duration::from_millis(60)); // Reduced from 500ms, humans shouldn't have to wait that long
    }

    println!("----- bonus -----");

    bonus();
}


// demonstrates a more realistic scenario of spinning up a new thread per job
fn bonus() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));

    // the jobs get increasingly quicker, first thread should finish last
    for tread_count in 0..10 {
        let status_shared = status.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(30 * (10 - tread_count)));
            let mut job_status = status_shared.lock().unwrap();
            println!("new thread ({}), current jobs completed: {}", tread_count, job_status.jobs_completed);
            job_status.jobs_completed += 1;
        });
    }

    while status.lock().unwrap().jobs_completed < 10 {
        println!("[main thread] waiting... ");
        thread::sleep(Duration::from_millis(60));
    }
}
