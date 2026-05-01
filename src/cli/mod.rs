use std::io;
use crate::smtp::smtp_server::AuthMechanism;

pub fn cli_smtp(auth_mechs: Vec<AuthMechanism>) -> Result<AuthMechanism, Box<dyn std::error::Error>> {
    if auth_mechs.is_empty() {
        return Err("Tidak ada metode autentikasi yang tersedia".into());
    }

    println!("Server supports authentication:\n");

    for (i, auth) in auth_mechs.iter().enumerate() {
        match auth {
            AuthMechanism::Plain => println!("[{}] PLAIN  -> Email + Password (base64)", i),
            AuthMechanism::Login => println!("[{}] LOGIN  -> Email + Password (challenge based)", i),
            AuthMechanism::XOAuth => println!("[{}] XOAUTH -> OAuth 1.0 token (legacy)", i),
            AuthMechanism::XOAuth2 => println!("[{}] XOAUTH2 -> OAuth 2.0 access token", i),
            AuthMechanism::OAuthBearer => println!("[{}] OAUTHBEARER -> OAuth 2.0 bearer token (RFC 7628)", i),
            AuthMechanism::PlainClientToken => println!("[{}] PLAIN-CLIENTTOKEN -> Google client token auth", i),
            AuthMechanism::Unknown(name) => println!("[{}] {} -> Unknown mechanism", i, name),
        }
    }

    println!("\nChoose authentication method by number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let choice: usize = input
        .trim()
        .parse()
        .map_err(|_| format!("'{}' Invalid input", input.trim()))?;

    // usize tidak perlu cek < 0
    if choice >= auth_mechs.len() {
        return Err(format!("Expected 0 - {}", auth_mechs.len() - 1).into());
    }

    // into_iter().nth() untuk move keluar dari Vec
    auth_mechs
        .into_iter()
        .nth(choice)
        .ok_or_else(|| "Index tidak ditemukan".into())
}