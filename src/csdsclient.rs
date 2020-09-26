#[allow(dead_code)]
use crate::request::{do_get_req, HttpResult};
// use crate::structs::Config;
use hyper::body::to_bytes;
use lru::LruCache;
use serde_json::{from_slice, from_str, to_string};

#[allow(dead_code)]
type GetAllResult = HttpResult<Vec<CsdsDomain>>;

#[allow(dead_code)]
pub struct CsdsClient {
    cache: LruCache<String, String>,
    account_id: String,
    csds_domain: String, // conf: Config,
}

impl CsdsClient {
    pub fn new(account_id: String, csds_domain: String) -> Self {
        CsdsClient {
            cache: LruCache::new(20),
            account_id,
            csds_domain,
        }
    }

    #[allow(dead_code)]
    pub fn convert(_base_uris: String) -> Vec<u32> {
        // not implemented
        vec![1]
    }

    #[allow(dead_code)]
    pub async fn getll(&mut self) -> GetAllResult {
        let acc_id = &self.account_id;
        // pub async fn getAll(&self) -> impl Future<Output = Domain> {
        let mut cached_domains = self.cache.get(acc_id);
        if cached_domains.is_none() {
            let domains = csds_url(&self.csds_domain, &self.account_id);
            let res = do_get_req(domains).await?;
            let body = to_bytes(res.into_body()).await?;
            let domains: Vec<CsdsDomain> = from_slice(&body)?;
            self.cache.put(acc_id.to_string(), to_string(&domains)?);
            cached_domains = self.cache.get(&self.account_id);
        }
        let data = cached_domains.unwrap();
        let domains: Vec<CsdsDomain> = from_str(&data)?;
        Ok(domains)
    }
}

#[allow(dead_code)]
fn csds_url(csds_base_domain: &String, account_id: &String) -> String {
    format!(
        "https://{}/api/account/{}/service/baseURI.json?version=1.0",
        csds_base_domain, account_id
    )
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CsdsDomain {
    pub service: String, // liveEngageUI | visitorFeed | etool
    pub account: String, // EXAMPLE123
    pub base_uri: String, // lo.le1.liveperson.net | lo.v-feed.liveperson.net | z2.etool.liveperson.net
}
