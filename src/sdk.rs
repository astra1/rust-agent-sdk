//! Minimal Agent SDK implemenation
//! Provides an async connect and methods for issuing the supported commands.

#[allow(dead_code)]
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
    csds_client: CsdsClient,
    // userId: u32,
    // agentId: u32,
}

#[allow(dead_code)]
impl AgentSdk {
    pub fn new(config: Config) -> Result<Self> {
        let merged_conf = Config {
            account_id: config.account_id,
            username: config.username,
            password: config.password,
            assertion: config.assertion,
            ..Config::default()
        };

        const EMPTY_STR: String = String::new();

        // verify that an accountId is present
        if merged_conf.account_id == EMPTY_STR {
            return Err(SdkError::MissingAccountId);
        }

        // verify that we have all the pieces to login
        if merged_conf.token == EMPTY_STR
            && (merged_conf.username == EMPTY_STR || merged_conf.password == EMPTY_STR )
            && (merged_conf.username == EMPTY_STR
                || merged_conf.app_key == EMPTY_STR
                || merged_conf.secret == EMPTY_STR
                || merged_conf.access_token == EMPTY_STR
                || merged_conf.access_token_secret == EMPTY_STR)
        {
            return Err(SdkError::MissingAuthData);
        }

        // initialize the WS requests - add them as functions to this object
        let _res = register_requests(consts::REQUESTS.to_vec());

        // create the CSDS client

        // connect

        let conf_rc = Rc::new(merged_conf);
        let account_id = conf_rc.clone().account_id.to_string();
        let csds_domain = &conf_rc.clone().csds_domain;

        Ok(AgentSdk {
            csds_client: CsdsClient::new(account_id, csds_domain.to_string()),
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
    pub fn get_bearer_token() {
        unimplemented!();
    }

    // Start a periodic call to refresh the HTTP session, thus prolonging the lifetime of the bearer token
    // The default time for the interval of call is 10 minutes
    pub fn start_periodic_refresh_session() {
        unimplemented!();
    }

    // Asynchronously refresh the HTTP session of the agent, thus prolonging the bearer token lifetime
    pub fn refresh_session() -> Result<()> {
        Ok(())
    }

    // gets a bearer token from agentVEP
    fn login() -> Result<()> {
        Ok(())
    }

    fn login_and_reconnect() -> Result<()> {
        Ok(())
    }

    fn init() {}

    // type UMS request type
    fn create_and_send_request() -> Result<()> {
        // transform. figure out

        // create the request

        // send the request
        Ok(())
    }

    // Reports if an async task is successful or not
    fn emit_status_events() {}

    // Handle CSDS calls and emit specific events based on error
    fn handle_csds(_location: String) -> Result<()> {
        Ok(())
    }

    // Handle login calls and emit specific events based on error
    fn handle_login() -> Result<()> {
        Ok(())
    }

    // Handle login and reconnect and emit specific events base on error
    fn handle_login_and_reconnect() -> Result<()> {
        Ok(())
    }

    // Handle Refresh Session calls
    fn handle_refresh_session_call() -> Result<()> {
        Ok(())
    }

    fn pre_check_refresh_session() -> Option<String> {
        Some("test".to_uppercase())
    }

    fn handle_message() {
        unimplemented!();
    }

    fn handle_socket_created() {
        unimplemented!();
    }

    fn queue_for_response_and_send() -> Result<()> {
        Ok(())
    }

    fn queue_for_response(_id: String) {
        unimplemented!();
    }

    fn check_for_errors() {
        unimplemented!();
    }

    fn handle_closed() {
        unimplemented!();
    }

    fn notify_socket_closed_failure() {
        unimplemented!();
    }

    fn handle_notification() {
        unimplemented!();
    }

    fn handle_response() {
        unimplemented!();
    }

    fn handle_outcome() {
        unimplemented!();
    }

    fn notify_outcome() {
        unimplemented!();
    }

    fn deque_message() {
        unimplemented!();
    }

    fn transform_msg() {
        unimplemented!();
    }
}

#[allow(dead_code)]
struct Domain {
    agent_vep: String,
}

fn register_requests<String>(_ws_requests: Vec<String>) -> Result<()> {
    Ok(())
}
