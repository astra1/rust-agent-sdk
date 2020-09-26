// use crate::csdsclient::CsdsDomain;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseUri {
    service: String,
    account: String,
    base_uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub std_ttl: u32,
    pub checkperiod: u32,
    pub jwt: String,
    pub csrf: String,
    pub account_id: String,
    pub username: String,
    pub password: String,
    pub token: String,     // a bearer token instead of username and password
    pub user_id: String,    // the user id - mandatory when using token as authentication method
    pub assertion: String, // a SAML assertion to be used instead of token or username and password (todo: check for XML https://knowledge.liveperson.com/security-regulations-login-sso-unified-login.html)
    pub app_key: String, // oauth1 keys needed (with username) to be used instead of assertion or token or username and password
    pub secret: String,
    pub access_token: String,
    pub access_token_secret: String,
    pub csds_domain: String,      // override the CSDS domain if needed
    pub request_timeout: u32,     // default to 10000 milliseconds
    pub error_check_interval: u32, // defaults to 1000 milliseconds
    pub api_version: usize, // Messaging API version - defaults to 2 (version 1 is not supported anymore)
    pub refresh_session_interval: u32,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            access_token: String::new(),
            access_token_secret: String::new(),
            account_id: String::new(),
            api_version: 0,
            app_key: String::new(),
            assertion: String::new(),
            jwt: String::new(),
            std_ttl: 60,
            csrf: String::new(),
            checkperiod: 0,
            // csdsDomain: "adminlogin.liveperson.net".to_string(),
            csds_domain: String::new(),
            error_check_interval: 30,
            password: String::new(),
            refresh_session_interval: 30_000,
            request_timeout: 30_000,
            secret: String::new(),
            user_id: String::new(),
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
