use std::sync::{Arc, Mutex, mpsc::Receiver};

use crate::{email::Email, queue::Queue};

#[derive(Debug)]
pub struct AppState {
    pub has_works: bool,
    pub total_works: u32,
    pub queue: Arc<Mutex<Queue>>,
}

// impl Default for AppState {
//     fn default(receiverEmail : Receiver<Email>) -> Self {
//         Self {
//             has_works: false,
//             total_works: 0,
//             queue: Queue::default(receiverEmail),
//         }
//     }
// }

impl AppState {

     pub fn new(queue : Arc<Mutex<Queue>>) -> Self {
        Self {
            has_works: false,
            total_works: 0,
            queue,
        }
    }

    pub fn enqueue(&mut self) {
        self.add_total_works();
    }

    pub fn dequeue(&mut self) {
        self.decrease_total_works();
    }

    fn add_total_works(&mut self) {
        if !self.has_works {
            self.has_works = true;
        }
        self.total_works = self.queue.lock().unwrap().get_total_work() as u32;
    }

    fn decrease_total_works(&mut self) {
        self.total_works = self.queue.lock().unwrap().get_total_work() as u32;
        if self.total_works == 0 {
            self.has_works = false
        }
    }

}
