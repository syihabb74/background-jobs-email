use std::sync::atomic::AtomicBool;

pub mod app_state;
pub mod cli;
pub mod email;
pub mod queue;
pub mod signaling;
pub mod smtp;
pub mod thread_pool;
pub mod uds;
pub type Closure = Box<dyn 'static + Fn(&mut Vec<String>, String)>;

pub static WILL_SHUTDOWN: AtomicBool = AtomicBool::new(false);
