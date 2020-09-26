use crate::request::do_post_req;
use hyper::client::HttpConnector;
use hyper::Request;
use hyper::{body::to_bytes, Body, Client, Method, Response};
use hyper_tls::HttpsConnector;
use serde_json::{from_slice, to_vec};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type HttpClient = Client<HttpsConnector<HttpConnector>>;
type Result<T> = std::result::Result<T, Error>;

use crate::structs::{Config, Login};

pub async fn login(config: &Config) -> Result<Login> {
    let url = format!(
        "https://{}/api/account/{}/login?v=1.3",
        config.csdsDomain, config.accountId
    );

    let _request = Request::builder().method(Method::POST).uri(&url); //.header(key: K, value: V);
    let req_body;

    if config.username != "" && config.password != "" {
        let loginBody = LoginPairBody {
            username: config.username.to_string(),
            password: config.password.to_string(),
        };
        req_body = Body::from(to_vec(&loginBody).unwrap());
    } else if config.assertion != "" {
        // todo: grabbed from nodejs lib, need to check deeper
        // TODO: remove - this is a hack against the agent vep
        let samlBody = SamlBody {
            jwt: config.jwt.to_string(),
            assertion: config.assertion.to_string(),
        };
        req_body = Body::from(to_vec(&samlBody).unwrap());
    } else {
        let oauthBody = OauthBody {
            username: config.username.to_string(),
            appKey: config.appKey.to_string(),
            secret: config.secret.to_string(),
            accessToken: config.accessToken.to_string(),
            accessTokenSecret: config.accessTokenSecret.to_string(),
        };
        req_body = Body::from(to_vec(&oauthBody).unwrap());
    }

    let res_body = do_post_req(&url, req_body).await;
    let body_bytes = to_bytes(res_body.unwrap()).await?;
    let login_res: Login = from_slice(&body_bytes)?;

    serde::export::Ok(login_res)
}

pub async fn refresh_session(config: &Config) -> Result<()> {
    let url = format!(
        "https://{}/api/account/{}/refresh?v=1.3",
        config.csdsDomain, config.accountId
    );
    let csrf_body = CsrfBody {
        csrf: config.csrf.to_string(),
    };
    let req_body = Body::from(to_vec(&csrf_body)?);

    do_post_req(&url, req_body).await?;
    Ok(())
}

pub async fn compile_error(
    _base_error_message: String,
    _res: &mut Response<Body>,
    _body: Body,
) -> Result<()> {
    Ok(())
    // if res.status().is_client_error() || res.status().is_server_error() {
    //     let bytes = to_bytes(res.into_body()).await.unwrap();
    //     let error_message = String::from_utf8_lossy(&bytes);
    //     return Err(format!("{}: {} {}", base_error_message, error_message, res.status()).into());
    // } else if res.status().is_success() == false {
    //     let bytes = to_bytes(res.into_body()).await?;
    //     let error_message = String::from_utf8_lossy(&bytes);
    //     return Err(format!(
    //         "{}: {}",
    //         base_error_message.to_string(),
    //         error_message.to_string()
    //     )
    //     .into());
    // } else {
    //     return Err(format!(
    //         "{}: {}",
    //         base_error_message.to_string(),
    //         String::from("test err")
    //     )
    //     .into());
    // }
    // return None;
}

#[derive(Serialize, Deserialize)]
struct LoginPairBody {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct SamlBody {
    jwt: String,
    assertion: String,
}

#[derive(Serialize, Deserialize)]
struct OauthBody {
    username: String,
    appKey: String,
    secret: String,
    accessToken: String,
    accessTokenSecret: String,
}

#[derive(Serialize, Deserialize)]
struct CsrfBody {
    csrf: String,
}
