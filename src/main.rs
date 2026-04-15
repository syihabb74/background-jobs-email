use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

use background_jobs::{app_state::AppState, queue::Queue, signaling, uds::UnixServer};

fn main() {
    let graceful_shutdown = signaling::graceful_shutdown();
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Mutex::new(Queue::new(rx)));
    let state_app = Arc::new(Mutex::new(AppState::new(Arc::clone(&queue))));
    let uds_state_app_cloned = Arc::clone(&state_app);
    

    let mut server = UnixServer::build(String::from("/tmp/server_bg_jobs.sock"));
    let run = server.deploy_uds();
    match run {
        Ok(_) => {
            println!("Running");
        }
        Err(e) => {
            println!("{}", e)
        }
    }
    let server = thread::spawn(move || {
        let state = Arc::clone(&uds_state_app_cloned);
        server.listening(tx, state);
    });

    graceful_shutdown.join().unwrap();
}
