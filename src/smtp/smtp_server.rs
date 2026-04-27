use std::{io::{BufRead, BufReader, Read, Write}, sync::Arc};

use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use rustls_pki_types::ServerName;

pub struct SmtpConfig {
    host: &'static str,
    port: u16,
}

impl SmtpConfig {
    pub fn builder(host: &'static str, port: u16) -> Self {
        Self { host, port }
    }

    pub fn connect<T>(&self, stream: T) -> LiveSmtp<T>
    where
        T: Read + Write,
    {
        LiveSmtp { stream }
    }
}

pub struct LiveSmtp<T: Read + Write> {
    stream: T,
}

impl <T : Read + Write>  LiveSmtp<T> {

    pub fn communicating(&mut self, cmd : &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        Self::write_cmd(&mut self.stream, cmd)?;

        let mut buff_reader = BufReader::new(&mut self.stream);
        let mut response = String::new();
        buff_reader.read_line(&mut response)?;

        Ok(())
    }

    pub fn write_cmd(stream: &mut impl Write, cmd: &[u8]) -> Result<(),  Box<dyn std::error::Error>> {
        let sending = stream.write_all(cmd)?;
        Ok(sending)
    }



    pub fn upgrade_tls(self, host : &str) -> Result<LiveSmtp<StreamOwned<ClientConnection, T>>, Box<dyn std::error::Error>> {

        let mut root_store = RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        let config = Arc::new(
            ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth()
        );

        let server_name = ServerName::try_from(host)?
                                          .to_owned();

        let conn = ClientConnection::new(config, server_name)?;

        Ok(LiveSmtp {
            stream: StreamOwned::new(conn, self.stream),
        })

    }

}
