use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseUri {
    service: String,
    account: String,
    baseURI: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub stdTTL: u32,
    pub checkperiod: u32,

    pub accountId: u32,
    pub username: String,
    pub password: String,
    pub token: String,     // a bearer token instead of username and password
    pub userId: String,    // the user id - mandatory when using token as authentication method
    pub assertion: String, // a SAML assertion to be used instead of token or username and password
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
