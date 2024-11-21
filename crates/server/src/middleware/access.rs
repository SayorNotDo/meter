use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};

use tower::{Layer, Service};
use uuid::Uuid;

use crate::{constant, errors::AppResponseError, service::permission, state::AppState};

#[derive(Clone)]
pub struct AccessLayer;

impl<S> Layer<S> for AccessLayer {
    type Service = AccessMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AccessMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AccessMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for AccessMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let uri = req.uri().clone();
        let method = req.method().clone();
        let headers = req.headers().clone();
        let state = req
            .extensions()
            .get::<AppState>()
            .cloned()
            .expect("Failed to get context...");
        let uid = match req.extensions().get::<Uuid>() {
            Some(i) => i.clone(),
            None => Uuid::nil(),
        };
        let future = self.inner.call(req);
        Box::pin(async move {
            /* Bypass api which is in WHITE_LIST */
            if constant::ACCESS_WHITE_LIST
                .iter()
                .any(|route| uri.path().starts_with(route))
            {
                return future.await;
            }
            /* access check main logic */

            let project_id = match headers.get(constant::PROJECT_ID) {
                Some(value) => match value.to_str().ok().and_then(|s| s.parse::<i32>().ok()) {
                    Some(id) => id,
                    None => {
                        let resp = build_error_response(
                            StatusCode::BAD_REQUEST,
                            "Missing or Invalid ProjectId header",
                        );
                        return Ok(resp);
                    }
                },
                None => {
                    let resp =
                        build_error_response(StatusCode::BAD_REQUEST, "Missing ProjectId header");
                    return Ok(resp);
                }
            };

            match permission::check_user_permission(
                &state,
                &uid,
                &project_id,
                uri.path(),
                method.as_str(),
            )
            .await
            {
                Ok(true) => {
                    return future.await;
                }
                Ok(false) => {
                    let resp = build_error_response(StatusCode::FORBIDDEN, "Access denied");
                    Ok(resp)
                }
                Err(e) => {
                    let resp = build_error_response(
                        StatusCode::FORBIDDEN,
                        &format!("Permission check failed: {e}"),
                    );
                    Ok(resp)
                }
            }
        })
    }
}

fn build_error_response(status: StatusCode, message: &str) -> Response {
    let err_body = Body::from(
        serde_json::to_string(&AppResponseError::new(
            "FORBIDDEN_ERROR".to_string(),
            message,
            None,
            vec![],
        ))
        .expect("Parse failure..."),
    );

    Response::builder()
        .status(status)
        .body(err_body)
        .expect("Build error response failure...")
}
