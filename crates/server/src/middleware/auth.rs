use std::task::{Context, Poll};

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use tower::{Layer, Service};

use crate::errors::AppResponseError;
use crate::state::AppState;
use crate::utils::claim::UserClaims;
use crate::{constant, service};

#[derive(Clone)]
pub struct AuthLayer;

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let uri = req.uri().clone();
        let headers = req.headers().clone();
        let state = req
            .extensions()
            .get::<AppState>()
            .cloned()
            .expect("Failed to get ctx...");
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let mut err_msg = String::new();

        Box::pin(async move {
            // Bypass api which is in WHITE_LIST
            if constant::WHITE_LIST
                .iter()
                .any(|route| uri.path().starts_with(route))
            {
                return inner.call(req).await;
            }
            let mut auth_str = String::new();
            match headers.get(constant::AUTHORIZATION) {
                Some(auth_header) => {
                    if let Ok(auth) = auth_header.to_str() {
                        auth_str = auth.to_string();
                    }
                }
                None => err_msg = "Headers field `Authorization` not found".to_string(),
            }
            if auth_str.starts_with(constant::BEARER) {
                let token = auth_str[6..].trim();
                match UserClaims::decode(token, &constant::ACCESS_TOKEN_DECODE_KEY) {
                    Ok(user_claims) => {
                        if service::session::check(&state.redis, &user_claims.claims)
                            .await
                            .is_ok()
                        {
                            req.extensions_mut().insert(user_claims.claims.uid);
                            return inner.call(req).await;
                        }
                    }
                    Err(err) => {
                        err_msg = err.to_string();
                    }
                }
            }
            // Authenticate failed
            let resp = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from(
                    serde_json::to_string(&AppResponseError::new(
                        "UNAUTHORIZED_ERROR".to_string(),
                        err_msg,
                        None,
                        vec![],
                    ))
                    .unwrap(),
                ))
                .unwrap();
            Ok(resp)
        })
    }
}
