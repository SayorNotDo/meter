use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, MatchedPath, Request},
    http::StatusCode,
    RequestPartsExt,
};
use serde_json::{json, Value};

pub struct CustomJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for CustomJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        let path = parts
            .extract::<MatchedPath>()
            .await
            .map(|path| path.as_str().to_owned())
            .ok();

        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let payload = json!({
                    "message": rejection.body_text(),
                    "origin": "custom_extractor",
                    "path": path,
                });
                Err((rejection.status(), axum::Json(payload)))
            }
        }
    }
}
