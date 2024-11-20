use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};

use tower::{Layer, Service};
use tracing::warn;
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

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
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
        let mut err_msg = String::new();
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            /* Bypass api which is in WHITE_LIST */
            if constant::ACCESS_WHITE_LIST
                .iter()
                .any(|route| uri.path().starts_with(route))
            {
                return inner.call(req).await;
            }
            /* access check main logic */
            if let Some(param) = headers.get(constant::PROJECT_ID) {
                if let Ok(parma_str) = param.to_str() {
                    if let Ok(project_id) = parma_str.parse::<i32>() {
                        /* 检查当前请求用户是否拥有对应接口所需要的权限
                           权限校验校验提供参数：
                           1.用户ID 通过Token解析获得
                           2.项目ID 请求头参数
                        */
                        match permission::check_user_permission(
                            &state,
                            &uid,
                            &project_id,
                            uri.path(),
                            method.as_str(),
                        )
                        .await
                        {
                            Ok(access) if access => {
                                req.extensions_mut().insert(project_id);
                                return inner.call(req).await;
                            }
                            Ok(_) => err_msg = "Access denied".to_string(),
                            Err(e) => err_msg = e.to_string(),
                        }
                    }
                }
            } else {
                err_msg = "projectId is required".to_string();
            }
            /* Access denied */
            let resp = Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::from(
                    serde_json::to_string(&AppResponseError::new(
                        "FORBIDDEN_ERROR".to_string(),
                        err_msg,
                        None,
                        vec![],
                    ))
                    .expect("Parse failure..."),
                ))
                .expect("Build response body failure...");
            Ok(resp)
        })
    }
}
