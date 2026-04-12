use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

use background_jobs::{WILL_SHUTDOWN, app_state::AppState, signaling, uds::UnixServer};

fn main() {
    let graceful_shutdown = signaling::graceful_shutdown();
    let state_app = Arc::new(Mutex::new(AppState::default()));
    let uds_state_app_cloned = Arc::clone(&state_app);
    let worker_state_app_cloned = Arc::clone(&state_app);
    let (tx, rx) = mpsc::channel();

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

    let worker = thread::spawn(move || {
        loop {
            match rx.recv_timeout(Duration::from_millis(500)) {
                Ok(email) => {
                    let state_clone = Arc::clone(&worker_state_app_cloned);
                    thread::spawn(move || {
                        {
                            let mut state = state_clone.lock().unwrap();
                            state.enqueue(email);
                        }
                        thread::sleep(Duration::from_millis(3000));
                        {
                            let mut state = state_clone.lock().unwrap();
                            state.dequeue();
                        }
                    });
                }
                Err(_) => {
                    println!("Nyangkut di worker")
                }
            }

            if WILL_SHUTDOWN.load(std::sync::atomic::Ordering::Relaxed) {
                let state = state_app.lock().unwrap();
                if state.total_works == 0 && !state.has_works {
                    break;
                }
            }
        }
    });

    server.join().unwrap();
    worker.join().unwrap();
    graceful_shutdown.join().unwrap();
}
