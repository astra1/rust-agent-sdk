use crate::structs::Config;
use hyper::{body::to_bytes, client::HttpConnector, Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;
use lru::LruCache;
use serde_json::{from_slice, from_str, to_string};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type HttpClient = Client<HttpsConnector<HttpConnector>>;
type Result<T> = std::result::Result<T, Error>;

type GetAllResult = Result<Vec<Domain>>;

pub struct CSDSClient {
    cache: LruCache<String, String>,
    conf: Config,
    csdsDomain: String,
}

impl CSDSClient {
    // todo: check let client = init_client();
    // fn new(&mut self, conf: Config) -> Self {
    //     let defaultConfig: Config = Default::default();
    //     let stdTTL = &defaultConfig.stdTTL;
    //     Self {
    //         cache: LruCache::new(10),
    //         conf: Config {
    //             stdTTL: 600,
    //             ..conf
    //         },
    //         getAll: getAll(&self),
    //         csdsDomain: "adminlogin.liveperson.net",
    //     }
    // }

    pub fn convert(baseURIs: String) -> Vec<u32> {
        // not implemented
        vec![1]
    }

    pub async fn getAll(&self) -> GetAllResult {
        // pub async fn getAll(&self) -> impl Future<Output = Domain> {
        let mut cachedDomains = &self.cache.get(&self.conf.accountId.to_string());
        if cachedDomains.is_none() {
            let domains_url = url_patterns(&self.csdsDomain, &self.conf.accountId);
            let res = do_get_req(&domains_url, &init_client()).await?;
            let body = to_bytes(res.into_body()).await?;
            let domains: Vec<Domain> = from_slice(&body)?;
            self.cache
                .put(self.conf.accountId.to_string(), to_string(&domains)?);
            cachedDomains = &self.cache.get(&self.conf.accountId.to_string());
        }
        let data = cachedDomains.unwrap();
        let domains: Vec<Domain> = from_str(&data)?;
        Ok(domains)
    }
}

fn init_client() -> HttpClient {
    let https = HttpsConnector::new();
    Client::builder().build::<_, Body>(https)
}

fn url_patterns(csds_domain: &str, account_id: &u32) -> String {
    format!(
        "https://{}/api/account/{}/service/baseURI.json?version=1.0",
        csds_domain, account_id
    )
}

async fn do_get_req(uri: &str, client: &HttpClient) -> Result<Response<Body>> {
    let request = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty());
    let res = client.request(request.unwrap()).await?;
    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
struct Domain {
    service: String, // liveEngageUI | visitorFeed | etool
    account: String, // EXAMPLE123
    baseURI: String, // lo.le1.liveperson.net | lo.v-feed.liveperson.net | z2.etool.liveperson.net
}

impl Default for Config {
    fn default() -> Config {
        Config {
            accessToken: String::new(),
            accessTokenSecret: String::new(),
            accountId: 0,
            apiVersion: 0,
            appKey: String::new(),
            assertion: String::new(),
            stdTTL: 60,
            checkperiod: 30,
            csdsDomain: String::from("adminlogin.liveperson.net"),
            errorCheckInterval: 30,
            password: String::new(),
            refreshSessionInterval: 30_000,
            requestTimeout: 30_000,
            secret: String::new(),
            userId: String::new(),
            username: String::new(),
            token: String::new(),
        }
    }
}
