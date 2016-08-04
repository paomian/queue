use std::sync::{Mutex,Condvar};

struct InnerQueue<T> {
    data_:Mutex<Vec<T>>,
    cond_:Condvar,
    size_:usize,
}

impl<T> InnerQueue<T> {
    pub fn new(size:usize) -> Self {
        InnerQueue {
            data_:Mutex::new(Vec::new()),
            cond_:Condvar::new(),
            size_:size,
        }
    }

    pub fn push(&self,x:T) {
        let mut data = self.data_.lock().unwrap();
        if self.size_ <= data.len() {
            data = self.cond_.wait(data).unwrap();
            data.push(x);
        } else if data.len() == 0 {
            data.push(x);
            self.cond_.notify_one();
        } else {
            data.push(x);
        }
    }

    pub fn pop(&self) -> Option<T> {
        let mut data = self.data_.lock().unwrap();
        if data.len() == 0 {
            data = self.cond_.wait(data).unwrap();
            return data.pop();
        } else if data.len() == self.size_ {
            let tmp = data.pop();
            self.cond_.notify_one();
            return tmp;

        } else {
            return data.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use std::sync::Arc;
        use std::thread;
        use super::InnerQueue;

        let queue = Arc::new(InnerQueue::<String>::new(3));

        loop {
            let data = queue.pop();
            queue.push(String::from("xxx"));
        }
    }
}
