use background_jobs::uds::UnixServer;


fn main() {
    let mut server = UnixServer::build(String::from("/tmp/server_bg_jobs.sock"));
    let run = server.deploy_uds();
    match run {
        Ok(_) => {
            println!("Running");
        },
        Err(e) => {
            println!("{}", e)
        }
    }

    server.listening();

}
