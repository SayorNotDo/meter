use crate::config::ConfigSMTP;
use crate::dto::Email;
use lettre::message::{self, MultiPart};
use lettre::transport::smtp::{authentication::Credentials, client::Tls};
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use lettre::{Message, SmtpTransport, Transport};
use tracing::info;

use crate::errors::AppResult;

pub type EmailClient = AsyncSmtpTransport<Tokio1Executor>;

pub trait EmailClientExt: Clone + Send + Sync {
    #[allow(dead_code)]
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
    AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)
        .expect("Failed to init email client...")
        .credentials(Credentials::new(
            config.username.clone(),
            config.password.clone(),
        ))
        .port(config.port)
        .tls(Tls::None)
        .build()
}

pub fn send(email: Message) -> AppResult {
    let config =
        crate::config::Config::parse("./config.toml").expect("Failed to parse configuration file");

    let credentials = Credentials::new(config.smtp.username, config.smtp.password);
    let sender = if config.smtp.tls_off {
        SmtpTransport::builder_dangerous(config.smtp.host)
            .port(config.smtp.port)
            .credentials(credentials)
            .build()
    } else {
        SmtpTransport::relay(&config.smtp.host)
            .unwrap()
            .port(config.smtp.port)
            .credentials(credentials)
            .build()
    };

    sender.send(&email)?;

    Ok(())
}

pub fn registered_inform(username: &str, password: &str) -> AppResult {
    let m = message::Message::builder()
        .subject("注册通知邮件")
        .from(
            "Nobody <nobody@domain.tld>"
                .parse()
                .expect("failed to parse sender's email address"),
        )
        .reply_to(
            "Receiver <nobody@domain.tld>"
                .parse()
                .expect("failed to parse receiver's email address"),
        )
        .to("To <nobody@domain.tld>"
            .parse()
            .expect("failed to parse to email address"))
        .multipart(MultiPart::alternative_plain_html(
            String::from("用户帐号密码"),
            format!(
                "<div><p><b>账号:</b> <i>{username}</i></p>
            <p><b>密码:</b> <i>{password}</i></p></div>",
            ),
        ))
        .expect("failed to generate email");

    send(m)
}
