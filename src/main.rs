use handler::Handler;
use lambda_http::{run, Error};
use lastfm::Client;
use std::sync::Arc;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
mod handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let cors_allow_origin = std::env::var("CORS_ALLOW_ORIGIN").unwrap_or("*".to_string());
    let username = std::env::var("LASTFM_USERNAME").expect("LASTFM_USERNAME not set");
    let api_key = std::env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY not set");
    let lastfm_client = Arc::from(
        Client::builder()
            .username(username)
            .api_key(api_key)
            .build(),
    );
    let handler = Handler::new(
        lastfm_client.clone(),
        Box::leak(Box::new(cors_allow_origin)),
    );

    run(handler).await
}
