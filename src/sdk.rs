//! Minimal Agent SDK implemenation
//! Provides an async connect and methods for issuing the supported commands.

use std::fmt;
use std::rc::Rc;

use crate::consts;
use crate::csdsclient::CsdsClient;
// use crate::external_services;
use crate::structs::Config;

#[derive(Debug)]
pub enum SdkError {
    MissingAccountId,
    // you must provide one of the following parameters:
    // * token
    // * user + password
    // * assertion
    // * appKey + secret + accessToken + accessTokenSecret
    MissingAuthData,
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SdkError::MissingAuthData => write!(f, "you must provide one of the folowing parameters: token, user/password, assertion, appKey/secret/accessToken/accessTokenSecret"),
            SdkError::MissingAccountId => write!(f, "Missing accountId param")
        }
    }
}

type Result<T> = std::result::Result<T, SdkError>;

// use tokio::net::{TcpStream, ToSocketAddrs};
// use crate::transport::{Transport};
// use ws::listen;
/// Establish connection with AgentSDK websocket server;
///
/// Requests are issued using the various methods of `Client`.
#[allow(dead_code)]
pub struct AgentSdk {
    config: Rc<Config>,
    // accountId: u32,

    // refreshSessionInterval: u32,
    csdsClient: CsdsClient,
    // userId: u32,
    // agentId: u32,
}

impl AgentSdk {
    pub fn new(config: Config) -> Result<Self> {
        let merged_conf = Config {
            accountId: config.accountId,
            username: config.username,
            password: config.password,
            assertion: config.assertion,
            ..Config::default()
        };

        const empty_str: String = String::new();

        // verify that an accountId is present
        if merged_conf.accountId == empty_str {
            return Err(SdkError::MissingAccountId);
        }

        // verify that we have all the pieces to login
        if merged_conf.token == empty_str
            && (merged_conf.username == empty_str || merged_conf.password == empty_str)
            && (merged_conf.username == empty_str
                || merged_conf.appKey == empty_str
                || merged_conf.secret == empty_str
                || merged_conf.accessToken == empty_str
                || merged_conf.accessTokenSecret == empty_str)
        {
            return Err(SdkError::MissingAuthData);
        }

        // initialize the WS requests - add them as functions to this object
        registerRequests(consts::requests.to_vec());

        // create the CSDS client

        // connect

        let conf_rc = Rc::new(merged_conf);
        let accountId = conf_rc.clone().accountId.to_string();
        let csdsDomain = &conf_rc.clone().csdsDomain;

        Ok(AgentSdk {
            csdsClient: CsdsClient::new(accountId, csdsDomain.to_string()),
            config: conf_rc,
        })
    }

    // Establish a WS connection for the agent. This also logins the agent.
    // You might want to call this function in the very beginning of the agent lifecycle
    pub fn connect(&self) -> Result<()> {
        // 1. get CSDS entries

        // let domains = handleCSDS("Connect#CSDS")?;

        // 2. login
        // let (loginDomains, token) = login(self.config.csdsDomain, "Connect#Login")?; // todo check for domains

        // 3. init

        // init(Config {
        //     domain: loginDomains.facadeMsg,
        //     token,
        //     ..self.config
        // });

        // listen(wsPath!(&self.config), |out| Transport {
        //     out,
        //     ping_timeout: None,
        //     expire_timeout: None,
        // })
        // .unwrap();
        Ok(())
    }

    // Reconnect WS connections for agents
    // You might want to call this function in an event of a failure, when the connection will need
    // to be re-established
    pub fn reconnect(_dont_regenerate_token: bool) -> Result<()> {
        Ok(())
    }

    // Dispose all connections, event listeners, and timers connected to this agent
    // You might want to call this function in the end of the agent life cycle
    pub fn disconnect() {}

    // Get the bearer token of the current agent
    pub fn getBearerToken() {}

    // Start a periodic call to refresh the HTTP session, thus prolonging the lifetime of the bearer token
    // The default time for the interval of call is 10 minutes
    pub fn startPeriodicRefreshSession() {}

    // Asynchronously refresh the HTTP session of the agent, thus prolonging the bearer token lifetime
    pub fn refreshSession() -> Result<()> {
        Ok(())
    }

    // gets a bearer token from agentVEP
    fn login() -> Result<()> {
        Ok(())
    }

    fn loginAndReconnect() -> Result<()> {
        Ok(())
    }

    fn init() {}

    // type UMS request type
    fn createAndSendRequest() -> Result<()> {
        // transform. figure out

        // create the request

        // send the request
        Ok(())
    }

    // Reports if an async task is successful or not
    fn emitStatusEvents() {}

    // Handle CSDS calls and emit specific events based on error
    fn handleCSDS(_location: String) -> Result<()> {
        Ok(())
    }

    // Handle login calls and emit specific events based on error
    fn handleLogin() -> Result<()> {
        Ok(())
    }

    // Handle login and reconnect and emit specific events base on error
    fn handleLoginAndReconnect() -> Result<()> {
        Ok(())
    }

    // Handle Refresh Session calls
    fn handleRefreshSessionCall() -> Result<()> {
        Ok(())
    }

    fn preCheckRefreshSession() -> Option<String> {
        Some("test".to_uppercase())
    }

    fn handleMessage() {}

    fn handleSocketCreated() {}

    fn queueForResponseAndSend() -> Result<()> {
        Ok(())
    }

    fn queueForResponse(_id: String) {}

    fn checkForErrors() {}

    fn handleClosed() {}

    fn notifySocketClosedFailure() {}

    fn handleNotification() {}

    fn handleResponse() {}

    fn handleOutcome() {}

    fn notifyOutcome() {}

    fn dequeMessage() {}

    fn transformMsg() {}
}

struct Domain {
    agentVep: String,
}

fn registerRequests<T>(_ws_requests: Vec<T>) -> Result<()> {
    Ok(())
}
