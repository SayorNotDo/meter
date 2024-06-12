use std::task::{Context, Poll};
use axum::{extract::Request, body::Body, response::Response};
use tower::{Layer, Service};

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
        S: Service<Request<Body>, Response=Response> + Send + 'static,
        S::Future: Send + 'static
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    fn call(&mut self, req: Request) -> Self::Future {
        let future = self.inner.call(req);
        Box::pin(async move {
            let resp: Response = future.await?;
            Ok(resp)
        })
    }
}