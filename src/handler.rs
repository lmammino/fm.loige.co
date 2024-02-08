use futures_util::future::BoxFuture;
use lambda_http::{Body, Request, Response};
use lambda_runtime::Service;
use lastfm::track::NowPlayingTrack;
use std::{
    sync::Arc,
    task::{Context, Poll},
};

pub trait LFMClient {
    fn playing(
        &self,
    ) -> impl std::future::Future<Output = Result<Option<NowPlayingTrack>, lastfm::errors::Error>>
           + std::marker::Send;
}

impl LFMClient for lastfm::Client<String, String> {
    async fn playing(&self) -> Result<Option<NowPlayingTrack>, lastfm::errors::Error> {
        self.now_playing().await
    }
}

#[derive(Debug, serde::Serialize)]
pub struct NowPlayingResponse {
    pub now_playing: Option<NowPlayingTrack>,
}

#[derive(Debug)]
pub struct Handler<LFM: LFMClient + Send + Sync> {
    pub lastfm_client: Arc<LFM>,
    pub cors_allow_origin: &'static str,
}

impl<LFM: LFMClient + Send + Sync> Clone for Handler<LFM> {
    fn clone(&self) -> Self {
        Self {
            lastfm_client: self.lastfm_client.clone(),
            cors_allow_origin: self.cors_allow_origin,
        }
    }
}

impl<LFM: LFMClient + Send + Sync> Handler<LFM> {
    pub fn new(lastfm_client: Arc<LFM>, cors_domain: &'static str) -> Self {
        Self {
            lastfm_client,
            cors_allow_origin: cors_domain,
        }
    }

    async fn do_call(&self, req: Request) -> Result<Response<Body>, lambda_runtime::Error> {
        // check the origin header and define the value for the CORS allow origin header
        let origin_header = req.headers().get("origin").and_then(|v| v.to_str().ok());
        let cors_allow_origin = match origin_header {
            Some(origin)
                if origin.starts_with("http://localhost:") || origin == "http://localhost" =>
            {
                origin
            }
            _ => self.cors_allow_origin,
        };

        let now_playing = self.lastfm_client.playing().await?;
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

impl<LFM: LFMClient + Send + Sync + 'static> Service<Request> for Handler<LFM> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct MockLFMClient;
    impl LFMClient for MockLFMClient {
        async fn playing(&self) -> Result<Option<NowPlayingTrack>, lastfm::errors::Error> {
            Ok(None)
        }
    }

    #[tokio::test]
    async fn test_handler_no_origin() {
        static CORS_ALLOW_ORIGIN: &str = "https://loige.co";
        let handler = Handler::new(Arc::new(MockLFMClient {}), CORS_ALLOW_ORIGIN);
        let req = Request::default();
        let resp = handler.do_call(req).await.unwrap();
        assert_eq!(resp.status(), 200);
        assert_eq!(
            resp.headers().get("Access-Control-Allow-Origin").unwrap(),
            CORS_ALLOW_ORIGIN
        );
    }

    #[tokio::test]
    async fn test_handler_localhost_origin() {
        static CORS_ALLOW_ORIGIN: &str = "https://loige.co";
        static ORIGIN_HEADER: &str = "http://localhost:4321";
        let handler = Handler::new(Arc::new(MockLFMClient {}), CORS_ALLOW_ORIGIN);
        let mut req = Request::default();
        req.headers_mut()
            .insert("origin", ORIGIN_HEADER.parse().unwrap());
        let resp = handler.do_call(req).await.unwrap();
        assert_eq!(resp.status(), 200);
        assert_eq!(
            resp.headers().get("Access-Control-Allow-Origin").unwrap(),
            ORIGIN_HEADER
        );
    }

    #[tokio::test]
    async fn test_handler_other_origin() {
        static CORS_ALLOW_ORIGIN: &str = "https://loige.co";
        static ORIGIN_HEADER: &str = "https://example.com";
        let handler = Handler::new(Arc::new(MockLFMClient {}), CORS_ALLOW_ORIGIN);
        let mut req = Request::default();
        req.headers_mut()
            .insert("origin", ORIGIN_HEADER.parse().unwrap());
        let resp = handler.do_call(req).await.unwrap();
        assert_eq!(resp.status(), 200);
        assert_eq!(
            resp.headers().get("Access-Control-Allow-Origin").unwrap(),
            CORS_ALLOW_ORIGIN
        );
    }
}
