use crate::csdsclient::CsdsDomain;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseUri {
    service: String,
    account: String,
    baseURI: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub stdTTL: u32,
    pub checkperiod: u32,
    pub jwt: String,
    pub csrf: String,
    pub accountId: String,
    pub username: String,
    pub password: String,
    pub token: String,     // a bearer token instead of username and password
    pub userId: String,    // the user id - mandatory when using token as authentication method
    pub assertion: String, // a SAML assertion to be used instead of token or username and password (todo: check for XML https://knowledge.liveperson.com/security-regulations-login-sso-unified-login.html)
    pub appKey: String, // oauth1 keys needed (with username) to be used instead of assertion or token or username and password
    pub secret: String,
    pub accessToken: String,
    pub accessTokenSecret: String,
    pub csdsDomain: String,      // override the CSDS domain if needed
    pub requestTimeout: u32,     // default to 10000 milliseconds
    pub errorCheckInterval: u32, // defaults to 1000 milliseconds
    pub apiVersion: u32, // Messaging API version - defaults to 2 (version 1 is not supported anymore)
    pub refreshSessionInterval: u32,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            accessToken: String::new(),
            accessTokenSecret: String::new(),
            accountId: String::new(),
            apiVersion: 0,
            appKey: String::new(),
            assertion: String::new(),
            jwt: String::new(),
            stdTTL: 60,
            csrf: String::new(),
            checkperiod: 0,
            // csdsDomain: "adminlogin.liveperson.net".to_string(),
            csdsDomain: String::new(),
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    pub csrf: String,
    pub wsuk: String,
    pub config: AgentConfig,
    pub csds_collection_response: CsdsCollectionResponse,
    pub account_data: AccountData,
    #[serde(rename = "sessionTTl")]
    pub session_ttl: String,
    pub bearer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentConfig {
    pub login_name: String,
    pub user_id: String,
    pub user_privileges: Vec<i64>,
    pub server_current_time: i64,
    pub time_diff: i64,
    pub server_time_zone_name: String,
    #[serde(rename = "serverTimeGMTDiff")]
    pub server_time_gmtdiff: i64,
    #[serde(rename = "isLPA")]
    pub is_lpa: bool,
    pub is_admin: bool,
    pub account_time_zone_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsdsCollectionResponse {
    pub base_uris: Vec<BaseAgentUri>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseAgentUri {
    pub account: String,
    #[serde(rename = "baseURI")]
    pub base_uri: String,
    pub service: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
    pub agent_groups_data: AgentGroupsData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentGroupsData {
    pub items: Vec<Item>,
    pub revision: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub deleted: bool,
    pub name: String,
}
