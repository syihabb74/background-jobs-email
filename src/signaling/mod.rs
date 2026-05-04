use crate::WILL_SHUTDOWN;
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};
use std::thread::{self, JoinHandle};

// pub trait signal {

// }

pub fn graceful_shutdown() -> JoinHandle<()> {
    thread::spawn(move || {
        let mut signals = Signals::new([SIGINT, SIGTERM]).unwrap();
        for signal in signals.forever() {
            match signal {
                SIGINT => {
                    println!("SIGINT received");
                    WILL_SHUTDOWN.store(true, std::sync::atomic::Ordering::Relaxed);
                    break;
                }
                SIGTERM => {
                    println!("SIGTERM received");
                    WILL_SHUTDOWN.store(true, std::sync::atomic::Ordering::Relaxed);
                    break;
                }
                _ => println!("Signal not covered"),
            }
        }
    })
}
