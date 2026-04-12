use crate::{email::Email, queue::Queue};

#[derive(Debug)]
pub struct AppState {
    pub has_works: bool,
    pub total_works: u32,
    pub queue: Queue,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            has_works: false,
            total_works: 0,
            queue: Queue::default(),
        }
    }
}

impl AppState {
    pub fn enqueue(&mut self, email: Email) {
        self.queue.add_queue(email);
        self.add_total_works();
    }

    pub fn dequeue(&mut self) {
        self.queue.remove_queue();
        self.decrease_total_works();
    }

    fn add_total_works(&mut self) {
        if !self.has_works {
            self.has_works = true;
        }
        self.total_works = self.queue.get_total_work() as u32;
    }

    fn decrease_total_works(&mut self) {
        self.total_works = self.queue.get_total_work() as u32;
        if self.total_works == 0 {
            self.has_works = false
        }
    }

}
