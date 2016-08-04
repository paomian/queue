use super::InnerQueue;
use std::thread;
use std::thread::JoinHandle;
uset std::sync::Mutex;

struct ThreadPool {
    pool_:Mutex<Vec<JoinHandle>>,
}

struct Job {
    job_:Box<Fn>
}
