use garde::Validate;
use lettre::message::header;
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
            .header(header::ContentType::TEXT_HTML)
            .subject(value.subject.clone())
            .body(value.body.clone())?)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EmailTemplate {
    Register { username: String, password: String },
    ForgetPassword { username: String, password: String },
}

impl EmailTemplate {
    pub fn get(&self) -> (tera::Context, &'static str) {
        let mut ctx = tera::Context::new();
        match self {
            Self::Register { username, password } => {
                ctx.insert("username", username);
                ctx.insert("password", password);
                (ctx, "email/register.html")
            }
            Self::ForgetPassword { username, password } => {
                ctx.insert("username", username);
                ctx.insert("password", password);
                (ctx, "register.html")
            }
        }
    }
}
