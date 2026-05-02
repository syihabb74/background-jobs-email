use crate::{cli::prompt, smtp::smtp_server::SmtpCredential};

#[derive(Debug)]
pub enum AuthMechanism {
    Unknown(String),
    Plain,
    Login,
    XOAuth,
    XOAuth2,
    OAuthBearer,
    PlainClientToken
}

impl AuthMechanism {

    pub fn new(mech : &str) -> Self {
         match mech {
                    "PLAIN" => Self::Plain,
                    "PLAIN-CLIENTTOKEN" => Self::PlainClientToken,
                    "LOGIN" => Self::Login,
                    "XOAUTH" => Self::XOAuth,
                    "XOAUTH2" => Self::XOAuth2,
                    "OAUTHBEARER" => Self::OAuthBearer,
                    x => Self::Unknown(x.into())
                }
    }

    pub fn cli_display(&self, no: usize) {
        match self {
            AuthMechanism::Plain => println!("[{}] PLAIN  -> Email + Password (base64)", no),
            AuthMechanism::Login => println!("[{}] LOGIN  -> Email + Password (challenge based)", no),
            AuthMechanism::XOAuth => println!("[{}] XOAUTH -> OAuth 1.0 token (legacy)", no),
            AuthMechanism::XOAuth2 => println!("[{}] XOAUTH2 -> OAuth 2.0 access token", no),
            AuthMechanism::OAuthBearer => println!("[{}] OAUTHBEARER -> OAuth 2.0 bearer token (RFC 7628)", no),
            AuthMechanism::PlainClientToken => println!("[{}] PLAIN-CLIENTTOKEN -> Google client token auth", no),
            AuthMechanism::Unknown(name) => println!("[{}] {} -> Unknown mechanism", no, name),
        }
    }


    pub fn generate_credentials(&self) -> Result<SmtpCredential, Box<dyn std::error::Error>> {
        match self {
        AuthMechanism::Plain |
        AuthMechanism::PlainClientToken |
        AuthMechanism::Login => {
            let mut email = String::new();
            let mut password = String::new();
            prompt("Email", &mut email);
            prompt("Password", &mut password);
            Ok(SmtpCredential::new_email_password(email, password))
        }
        AuthMechanism::XOAuth |
        AuthMechanism::XOAuth2 => {
            let mut email = String::new();
            let mut token = String::new();
            prompt("Email", &mut email);
            prompt("OAuth Token", &mut token);
            Ok(SmtpCredential::new_oauth(email, token))
        }
        AuthMechanism::OAuthBearer => {
            let mut token = String::new();
            prompt("Bearer Token", &mut token);
            Ok(SmtpCredential::new_oauth_bearer(token))
        }
        AuthMechanism::Unknown(s) => Err(s.to_string().into())
        }
        // closure()
    }

}