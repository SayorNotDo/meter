use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};

use tower::{Layer, Service};

use crate::{constant, dao::permission::PermissionDao, errors::AppResponseError, state::AppState};

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
        let headers = req.headers().clone();
        let state = req
            .extensions()
            .get::<AppState>()
            .cloned()
            .expect("Failed to get context...");
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
            if let Some(_project_id) = headers.get(constant::PROJECT_ID) {
                let client = state
                    .pool
                    .get()
                    .await
                    .expect("failed to get db client with ctx...");
                let _perm_dao = PermissionDao::new(&client);
                /* 查询当前请求接口需要的权限 */
                /* 查询请求用户是否拥有该权限 */
            }
            /* Access denied */
            let resp = Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::from(
                    serde_json::to_string(&AppResponseError::new(
                        "FORBIDDEN".to_string(),
                        "",
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
