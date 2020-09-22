use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;

/// Error returned by most functions.
///
/// check error handling crate
///
/// For performance reasons, boxing is avoided in any hot path. For example, in
/// `parse`, a custom error `enum` is defined. This is because the error is hit
/// and handled during normal execution when a partial frame is received on a
/// socket. `std::error::Error` is implemented for `parse::Error` which allows
/// it to be converted to `Box<dyn std::error::Error>` todo: write about 'static.
pub type HttpError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type HttpClient = Client<HttpsConnector<HttpConnector>>;
pub type HttpResult<T> = std::result::Result<T, HttpError>;

pub async fn do_get_req(uri: &str) -> HttpResult<Response<Body>> {
    let client = init_client();
    let request = Request::builder()
        // todo: DRY
        .header(
            "user-agent",
            format!("RustAgentSDK/{}", env!("CARGO_PKG_VERSION")),
        )
        .header("content-type", "application/json")
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty());
    let res = client.request(request.unwrap()).await?;
    Ok(res)
}

pub async fn do_post_req(uri: &str, body: &Body) -> HttpResult<Response<Body>> {
    let client = init_client();
    let request = Request::builder()
        // todo: DRY
        .header(
            "user-agent",
            format!("RustAgentSDK/{}", env!("CARGO_PKG_VERSION")),
        )
        .header("content-type", "application/json")
        .method(Method::POST)
        .uri(uri)
        .body(*body);

    let res = client.request(request.unwrap()).await?;

    Ok(res)
}

fn init_client() -> HttpClient {
    let https = HttpsConnector::new();
    Client::builder().build::<_, Body>(https)
}
