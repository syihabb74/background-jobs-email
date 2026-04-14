use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

use background_jobs::{app_state::AppState, signaling, thread_pool::ThreadPool, uds::UnixServer, worker::worker};

fn main() {
    let graceful_shutdown = signaling::graceful_shutdown();
    let state_app = Arc::new(Mutex::new(AppState::default()));
    let uds_state_app_cloned = Arc::clone(&state_app);
    let worker_state_app_cloned = Arc::clone(&state_app);
    let (tx, rx) = mpsc::channel();
    let pool_thread = ThreadPool::new(4);
    

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

    // let worker = worker(rx, worker_state_app_cloned);
    server.join().unwrap();
    worker.join().unwrap();
    graceful_shutdown.join().unwrap();
}
