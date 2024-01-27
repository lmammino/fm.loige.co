use futures_util::future::BoxFuture;
use lambda_http::{Body, Request, Response};
use lambda_runtime::Service;
use lastfm::track::NowPlayingTrack;
use std::task::{Context, Poll};

#[derive(Debug, serde::Serialize)]
pub struct NowPlayingResponse {
    pub now_playing: Option<NowPlayingTrack>,
}

#[derive(Debug, Clone)]
pub struct Handler {
    pub lastfm_client: &'static lastfm::Client<String, String>,
    pub cors_allow_origin: &'static str,
}

impl Handler {
    pub fn new(
        lastfm_client: &'static lastfm::Client<String, String>,
        cors_domain: &'static str,
    ) -> Self {
        Self {
            lastfm_client,
            cors_allow_origin: cors_domain,
        }
    }

    async fn do_call(&self, req: Request) -> Result<Response<Body>, lambda_runtime::Error> {
        // check the origin header
        let origin = req.headers().get("origin").and_then(|v| v.to_str().ok());
        if origin.is_none() {
            return Ok(Response::builder()
                .status(400)
                .body(Body::Text("Missing origin header".to_string()))
                .map_err(Box::new)?);
        }
        let origin = origin.unwrap();
        let cors_allow_origin = if origin.starts_with("http://localhost:") {
            origin
        } else {
            self.cors_allow_origin
        };

        let now_playing = self.lastfm_client.now_playing().await?;
        let body = Body::Text(serde_json::to_string(&NowPlayingResponse { now_playing })?);
        let resp = Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .header("Cache-Control", "max-age=30")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .header("Access-Control-Allow-Methods", "OPTIONS,GET")
            .header("Access-Control-Allow-Origin", cors_allow_origin)
            .body(body)
            .map_err(Box::new)?;

        Ok(resp)
    }
}

impl Service<Request> for Handler {
    type Response = Response<Body>;
    type Error = lambda_runtime::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let clone = self.clone();
        Box::pin(async move { clone.do_call(req).await })
    }
}
