

pub mod uds {
    
    use std::{io::ErrorKind, os::unix::net::UnixListener};
    
        #[derive(Debug)]  
        pub struct UnixServer{
            path : String,
            listener : Option<UnixListener>
        }
        
        impl UnixServer {
            
            pub fn build(path : String) -> Self {
                Self{
                    path,
                    listener : None
                }        
            }
        
            pub fn deploy_uds(&mut self) -> Result<(), String> {
                
                if !self.path.contains(".sock") {
                    return Err(String::from("Invalid format file it should be .sock"))
                }

                let _ = std::fs::remove_file(&self.path);

                match UnixListener::bind(&self.path) {
                    Ok(uds) => {
                        println!("Unix domain socket already listener on file {}", self.path);
                        self.listener = Some(uds);
                        Ok(())
                    },
                    Err(e) => {
                        match e.kind() {
                            ErrorKind::AddrInUse => {
                                Err("File path being use".to_string())
                            },
                            ErrorKind::AlreadyExists => {
                                Err("Enum already exist file path being use".to_string())
                            },
                            _ => {
                                Err("Unknown error".to_string())
                            }
                        }
                    }
                }
            }

            fn disconnected(&mut self) {

               if let Some(_listener) = self.listener.take() {
                   println!("Listener closed");
               }

                match std::fs::remove_file(&self.path) {
                    Ok(_) => {
                        println!("File deleted sucessfully")
                    },
                    Err(err) => {
                        match err.kind() {
                            ErrorKind::NotFound => {
                                println!("File not found")
                            },
                            _ => {
                                println!("Another error")
                            }
                        }
                    }
                }

            }
        
        }

        impl Drop for UnixServer {
        fn drop(&mut self) {
            self.disconnected();
        }
    }

}



