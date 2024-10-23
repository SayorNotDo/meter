use crate::configure::smtp::ConfigSMTP;
use crate::constant::{EMAIL_ADDR, TEMPLATE_ENGINE};
use crate::dto::{Email, EmailTemplate};
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use tracing::info;

use crate::errors::AppResult;

pub type EmailClient = AsyncSmtpTransport<Tokio1Executor>;

pub trait EmailClientExt: Clone + Send + Sync {
    fn send_email(&self, email: &Email) -> impl std::future::Future<Output = AppResult>;
}

impl EmailClientExt for EmailClient {
    async fn send_email(&self, email: &Email) -> AppResult {
        let resp = self.send(Message::try_from(email)?).await?;
        info!("Sent email successfully code: {:?}", resp.code());
        Ok(())
    }
}

pub fn email_client_builder(config: &ConfigSMTP) -> EmailClient {
    AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
        .expect("Failed to init email client...")
        .credentials(Credentials::new(
            config.username.clone(),
            config.password.clone(),
        ))
        .port(config.port)
        .build()
}

pub async fn send(
    client: &EmailClient,
    template: &EmailTemplate,
    subject: &str,
    receiver: &str,
) -> AppResult {
    info!("Send: {subject} email to addr: {receiver}");
    let email = create(template, subject, receiver)?;
    client.send_email(&email).await?;
    Ok(())
}

pub fn create(template: &EmailTemplate, subject: &str, receiver: &str) -> AppResult<Email> {
    info!("Create the email object: {template:?}");
    Ok(Email::new(
        EMAIL_ADDR.to_string(),
        receiver.to_string(),
        subject.to_string(),
        TEMPLATE_ENGINE.render(template)?,
    ))
}
