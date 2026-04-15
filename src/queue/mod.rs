use std::{collections::VecDeque, sync::mpsc::Receiver};

use crate::{email::{Email}};




    #[derive(Debug)]
    pub struct Queue {
        pub queue : VecDeque<Email>,
        pub receiver : Receiver<Email>
       
    }

    impl Queue {
        pub fn new (receiver : Receiver<Email>) -> Self {
            Self { queue: VecDeque::new(), receiver }
        }

        pub fn add_queue (&mut self, email : Email) {
            
            self.queue.push_back(email);
            println!("{:?}", self.queue)

        }

        pub fn get_total_work (&self) -> usize {
            self.queue.len()
        }

        pub fn remove_queue(&mut self) {
            let email = self.queue.pop_front();
            println!("{:?}", self.queue);
            if let Some(email_exist) = email {
                email_exist.sending_email();
            }
        }

    }

