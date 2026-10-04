use hcloud::apis::configuration::Configuration;
use hcloud::apis::{locations_api, ssh_keys_api, Error};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const LOCATIONS_BODY: &str = r#"{"locations":[],"meta":{"pagination":{"last_page":1,"next_page":null,"page":1,"per_page":25,"previous_page":null,"total_entries":0}}}"#;

/// Serves a single canned HTTP response and returns a configuration pointing at it.
async fn serve_once(status: &str, body: &'static str) -> Configuration {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nRateLimit-Limit: 3600\r\nRateLimit-Remaining: 3599\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut request = Vec::new();
        while !request.ends_with(b"\r\n\r\n") {
            request.push(stream.read_u8().await.unwrap());
        }
        stream.write_all(response.as_bytes()).await.unwrap();
        stream.shutdown().await.unwrap();
    });

    let mut configuration = Configuration::new();
    configuration.base_path_mapping.insert(
        "https://api.hetzner.cloud/v1".to_owned(),
        format!("http://{addr}"),
    );
    configuration
}

#[tokio::test]
async fn success_returns_entity_and_headers() {
    let configuration = serve_once("200 OK", LOCATIONS_BODY).await;

    let (response, headers) =
        locations_api::list_locations_with_headers(&configuration, Default::default())
            .await
            .unwrap();

    assert!(response.locations.is_empty());
    assert_eq!(headers["ratelimit-limit"], "3600");
    assert_eq!(headers["ratelimit-remaining"], "3599");
}

#[tokio::test]
async fn success_without_body_returns_headers() {
    let configuration = serve_once("204 No Content", "").await;

    let ((), headers) = ssh_keys_api::delete_ssh_key_with_headers(
        &configuration,
        ssh_keys_api::DeleteSshKeyParams { id: 1 },
    )
    .await
    .unwrap();

    assert_eq!(headers["ratelimit-remaining"], "3599");
}

#[tokio::test]
async fn plain_call_still_returns_entity() {
    let configuration = serve_once("200 OK", LOCATIONS_BODY).await;

    let response = locations_api::list_locations(&configuration, Default::default())
        .await
        .unwrap();

    assert!(response.locations.is_empty());
}

#[tokio::test]
async fn error_response_keeps_headers() {
    let body = r#"{"error":{"code":"rate_limit_exceeded","message":"limit reached"}}"#;
    let configuration = serve_once("429 Too Many Requests", body).await;

    let err = locations_api::list_locations(&configuration, Default::default())
        .await
        .unwrap_err();

    let Error::ResponseError(content) = err else {
        panic!("expected a response error, got {err:?}");
    };
    assert_eq!(content.status, 429);
    assert_eq!(content.headers["ratelimit-remaining"], "3599");
}
