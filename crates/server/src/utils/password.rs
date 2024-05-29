use super::hash;
use crate::errors::{AppResult, invalid_input_error};
use tracing::debug;

pub async fn hash(password: String) -> AppResult<String> {
    let join_handle = tokio::task::spawn_blocking(move || hash::argon_hash(password));
    let password = join_handle.await??;
    Ok(password)
}

pub async fn verify(password: String, hash: String) -> AppResult {
    let join_handle = tokio::task::spawn_blocking(move || hash::argon_verify(password, hash));
    if let Err(e) = join_handle.await? {
        debug!("Failed to verify password: {e:?}");
        Err(invalid_input_error(
            "password",
            "The password is not correct.",
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_hash() {
        let password = "password";
        let hashed_password = hash(password.to_string()).await.unwrap();
        assert!(!hashed_password.is_empty());
    }


    #[tokio::test]
    pub async fn test_verify() {
        let password = "password";
        let hashed_password = hash(password.to_string()).await.unwrap();

        verify(password.to_string(), hashed_password).await.unwrap();
    }
}