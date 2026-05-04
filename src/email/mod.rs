use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub content: String,
}

impl Email {
    pub fn to_struct_single(buf: &[u8; 1024], n: usize) -> Result<Self, serde_json::Error> {
        serde_json::from_slice::<Email>(&buf[..n])
    }

    pub fn to_struct_batches(buf: &[u8; 1024], n: usize) -> Result<Vec<Self>, serde_json::Error> {
        serde_json::from_slice::<Vec<Email>>(&buf[..n])
    }

    pub fn sending_email(self) {
        println!("Email sending to {}", self.to);
    }
}
