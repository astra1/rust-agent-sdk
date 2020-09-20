use lru::LruCache;
use reqwest::Result;
use serde::{Deserialize, Serialize};
use shared::structs::{BaseUri, Config};
use std::collections::HashMap;
use url::{ParseError, Url};

struct CSDSClient {
    cache: LruCache,
    conf: Config,
    csdsDomain: String,

    getAll: fn(Future<T>),

    requestHandler: fn(),

    convert: fn(HashMap),
}

impl CSDSClient {
    fn new(conf: Config) -> Self {
        Self {
            cache,
            conf: Config {
                conf,
                ..Default::default()
            },
            csdsDomain: "adminlogin.liveperson.net",
        }
    }

    fn getAll(self) {
        let url: Url = urlPatterns!(self.csdsDomain, self.accountId);
        let cachedDomains = self.cache.get(self.conf.accountId);
        if cachedDomains.is_none() {
            let resp = reqwest::blocking::get(url)?.json::<HashMap<String, String>>()?;
        }
    }

    fn requestHandler(self, resp: HashMap<String, String>) {
        let mut domains = Vec::new(10);

        if (resp.get("baseURIs")) {
            // debug here
            domains = CSDSClient.convert(resp.get("baseURIs"));
        }

        if (domains.len() > 0) {
            self.cache.set(self.conf.accountId, domains)
        }
    }

    fn convert(baseURIs: String) -> Vec {
        // not implemented
        Vec::new(1)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            stdTTL: 60,
            checkperiod: 30,
            csdsDomain: "adminlogin.liveperson.net",
        }
    }
}

#[proc_macro]
pub fn urlPatterns(csdsDomain: String, accountId: u32) -> Url {
    Url::parse(format!(
        "https://${csdsDomain}/api/account/${accountId}/service/baseURI.json?version=1.0",
        csdsDomain, accountId
    ))
}
