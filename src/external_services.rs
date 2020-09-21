use hyper::{Body, Method, Request, Client, Response};
use serde::{Deserialize, Serialize};
use shared::structs::Config;
use url::Url;

    const USER_AGENT_HEADER: (String, String) = ("User-Agent", format!("RustAgentSDK/{}", env!("CARGO_PKG_VERSION")));
    const APP_JSON_HEADER: (String, String) = ("content-type", "application/json");

pub mod external_services {

    pub fn login(config: &Config) -> Result<()> {
        let url = Url::parse(format!(
            "https://{domain}/api/account/{accountId}/login?v=1.3",
            domain = config.domain,
            accountId = config.accountId
        ))
        .unwrap();

        let mut request = Request::builder().method(Method::POST).uri(&url); //.header(key: K, value: V);
        let mut body = Body::empty();

        if config.userName != "" && config.password != "" {
            body = Body::from(LoginPairBody {
                username: config.username,
                password: config.password,
            });
        } else if config.assertion {
            // todo: grabbed from nodejs lib, need to check deeper
            // TODO: remove - this is a hack against the agent vep
            body = Body::from(SamlBody {
                jwt: config.jwt,
                assertion: config.assertion,
            });
        } else {
            body = Body::from(OauthBody {
                username: config.username,
                appKey: config.appKey,
                secret: config.secret,
                accessToken: config.accessToken,
                accessTokenSecret: config.accessTokenSecret,
            })
        }

        
        request.header(&USER_AGENT_HEADER).header(&APP_JSON_HEADER).body(&body).expect("Can't send auth request");

        let client = Client::new();
        let resp = client.request(request).await?;
        println!(resp.body());
    }

    pub fn refresh_session(config: &Config) -> Result<()> {
        let url = Url::parse("https://{domain}/api/account/{accountId}/refresh?v=1.3", domain = config.domain, accountId = config.accountId);
        let req = Request::builder().header(&USER_AGENT_HEADER).header(&APP_JSON_HEADER).body(Body::from(CsrfBody {
            csrf: config.csrf
        }))?;

        let client = Client::new();
        let resp = client.request(req).await?;

        println!("Response: {}", resp.status());
    }

    pub fn compile_error(base_error_message: String, err: Error, res: Response, body: Body) -> Error {
        if res.status().is_client_error() || res.status().is_server_error() {
            return Err(format!("{}: {} {}", base_error_message, res.body(), res.status());
        } else if (body.error) {
            return Err(format!("{}: {}", base_error_message, body.error));
        } else if err {
            return Err(format!("{}: {}", base_error_message, err));
        }
        return None;
    }

    #[derive(Serialize, Deserialize)]
    struct LoginPairBody {
        username: String,
        password: String,
    }

    #[derive(Serialize, Deserialize)]
    struct SamlBody {
        jwt: String,
        assertion: bool,
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
}
