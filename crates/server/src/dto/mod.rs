use garde::Validate;
use lettre::Message;
use serde::{Deserialize, Serialize};

pub mod request;
pub mod response;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Email {
    #[garde(email)]
    pub from: String,
    #[garde(email)]
    pub to: String,
    #[garde(length(min = 1))]
    pub subject: String,
    #[garde(length(min = 1))]
    pub body: String,
}

impl Email {
    pub fn new(from: String, to: String, subject: String, body: String) -> Self {
        Self {
            from,
            to,
            subject,
            body,
        }
    }
}

impl TryFrom<&Email> for Message {
    type Error = anyhow::Error;

    fn try_from(value: &Email) -> Result<Self, Self::Error> {
        Ok(Message::builder()
            .from(value.from.parse()?)
            .to(value.to.parse()?)
            .subject(value.subject.clone())
            .body(value.body.clone())?)
    }
}
