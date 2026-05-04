use ch11_web_crawler::{Config, run};
use std::sync::Once;
use url::Url;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

static INIT: Once = Once::new();
fn init_tracing() {
    INIT.call_once(|| {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
            )
            .with_test_writer()
            .try_init();
    });
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn crawls_two_pages_via_link() {
    init_tracing();
    let server = MockServer::start().await;
    let body_root = format!(
        "<html><body><a href=\"{}/p2\">next</a></body></html>",
        server.uri()
    );
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body_root))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/p2"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>leaf</body></html>"),
        )
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().expect("tempdir");
    let cfg = Config {
        seeds: vec![Url::parse(&server.uri()).expect("seed URL")],
        db_path: dir.path().join("crawl.db"),
        max_pages: 10,
        workers: 2,
        max_depth: 2,
    };
    let written = run(cfg).await.expect("run failed");
    assert!(written >= 2, "expected at least 2 pages, got {written}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn retries_on_5xx_then_succeeds() {
    init_tracing();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(2)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body>ok</body></html>"),
        )
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().expect("tempdir");
    let cfg = Config {
        seeds: vec![Url::parse(&server.uri()).expect("seed URL")],
        db_path: dir.path().join("crawl.db"),
        max_pages: 5,
        workers: 1,
        max_depth: 0,
    };
    let written = run(cfg).await.expect("run failed");
    assert_eq!(written, 1);
}
