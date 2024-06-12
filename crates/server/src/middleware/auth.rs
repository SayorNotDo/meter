use std::task::{Context, Poll};

use axum::{body::Body, http::{Request, StatusCode}, response::Response};
use serde::Serialize;
use tower::{Layer, Service};

use crate::{constant, service};
use crate::dto::response::MessageResponse;
use crate::errors::AppResponseError;
use crate::state::AppState;
use crate::utils::claim::UserClaims;

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


#[derive(Serialize)]
pub struct ResponseBody {
    pub message: &'static str,
    pub data: &'static str,
}

impl ResponseBody {
    pub fn new(message: &'static str, data: &'static str) -> Self {
        ResponseBody { message, data }
    }
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
    where
        S: Service<Request<Body>, Response=Response> + Clone + Send + 'static,
        S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let uri = req.uri().clone();
        let headers = req.headers().clone();
        let state = req.extensions().get::<AppState>().cloned().unwrap();
        let future = self.inner.call(req);
        Box::pin(async move {
            // Bypass api which in WHITE_LIST
            if constant::WHITE_LIST.iter().any(|route| uri.path().starts_with(route)) {
                return future.await;
            }
            if let Some(auth_header) = headers.get(constant::AUTHORIZATION) {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with(constant::BEARER) {
                        let token = auth_str[6..].trim();
                        let user_claims = UserClaims::decode(token, &constant::ACCESS_TOKEN_DECODE_KEY).unwrap().claims;
                        if service::session::check(&state.redis, &user_claims).await.is_ok() {
                            return future.await;
                        }
                    }
                }
            }
            // Authenticate failed
            let resp = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from(
                    serde_json::to_string(&AppResponseError::new(
                        "UNAUTHORIZED_ERROR".to_string(),
                        "User unauthorized".to_string(),
                        None,
                        vec![]
                    )).unwrap(),
                )).unwrap();
            Ok(resp)
        })
    }
}