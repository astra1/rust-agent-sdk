struct Config {
    accountId: u32,
    username: str,
    password: str,
    token: str,     // a bearer token instead of username and password
    userId: str,    // the user id - mandatory when using token as authentication method
    assertion: str, // a SAML assertion to be used instead of token or username and password
    appKey: str, // oauth1 keys needed (with username) to be used instead of assertion or token or username and password
    secret: str,
    accessToken: str,
    accessTokenSecret: str,
    csdsDomain: str,         // override the CSDS domain if needed
    requestTimeout: u32,     // default to 10000 milliseconds
    errorCheckInterval: u32, // defaults to 1000 milliseconds
    apiVersion: u32, // Messaging API version - defaults to 2 (version 1 is not supported anymore)
    refreshSessionInterval: u32,
}
