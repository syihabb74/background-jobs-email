use crate::smtp::smtp_server::AuthMechanism;

pub fn cli_smtp(auth_mechs: &Vec<AuthMechanism>) {
    println!("Server supports authentication:\n");

    for (i, auth) in auth_mechs.iter().enumerate() {
        match auth {
            AuthMechanism::Plain => {
                println!("[{}] PLAIN  -> Email + Password (base64)", i);
            }
            AuthMechanism::Login => {
                println!("[{}] LOGIN  -> Email + Password (challenge based)", i);
            }
            AuthMechanism::XOAuth => {
                println!("[{}] XOAUTH -> OAuth 1.0 token (legacy)", i);
            }
            AuthMechanism::XOAuth2 => {
                println!("[{}] XOAUTH2 -> OAuth 2.0 access token", i);
            }
            AuthMechanism::OAuthBearer => {
                println!("[{}] OAUTHBEARER -> OAuth 2.0 bearer token (RFC 7628)", i);
            }
            AuthMechanism::PlainClientToken => {
                println!("[{}] PLAIN-CLIENTTOKEN -> Google client token auth", i);
            }
            AuthMechanism::Unknown(name) => {
                println!(
                    "[{}] {} -> Unknown mechanism (server specific / custom)",
                    i, name
                );
            }
        }
    }

    println!("\nChoose authentication method by number:");
}