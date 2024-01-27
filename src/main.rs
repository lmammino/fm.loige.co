use handler::Handler;
use lambda_http::{run, Error};
use lastfm::Client;
mod handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let cors_allow_origin = std::env::var("CORS_ALLOW_ORIGIN").unwrap_or("*".to_string());
    let username = std::env::var("LASTFM_USERNAME").expect("LASTFM_USERNAME not set");
    let api_key = std::env::var("LASTFM_API_KEY").expect("LASTFM_API_KEY not set");
    let lastfm_client = Client::builder()
        .username(username)
        .api_key(api_key)
        .build();
    let handler = Handler::new(
        Box::leak(Box::new(lastfm_client)),
        Box::leak(Box::new(cors_allow_origin)),
    );

    run(handler).await
}
