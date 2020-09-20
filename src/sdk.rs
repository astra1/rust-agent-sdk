//! Minimal Agent SDK implemenation
//! Provides an async connect and methods for issuing the supported commands.
//! Uses Mini-Redis as alma mater

use shared::structs::Config;
use std::time::Duration;
use tokio::net::{TcpStream, ToSocketAddrs};
use transport::{wsPath, Transport};
use ws::listen;
use csdsclient::CSDSClient;
/// Establish connection with AgentSDK websocket server;
///
/// Requests are issued using the various methods of `Client`.
#[allow(dead_code)]
pub struct SDK {
    config: Config,
    accountId: u32,
    refreshSessionInterval: u32,
    csdsClient: CSDSClient,

    userId: u32,

    subscribeExConversations: fn(),
    subscribeAgentsState: fn(),
    subscribeRoutingTasks: fn(),
    subscribeMessagingEvents: fn(),
    updateRoutingTaskSubscription: fn(),
    unsubscribeExConversations: fn(),
    setAgentState: fn(),
    getClock: fn(),
    getUserProfile: fn(),
    updateRingState: fn(),
    agentRequestConversation: fn(),
    updateConversationField: fn(),
    generateURLForDownloadFile: fn(),
    generateURLForUploadFile: fn(),
    publishEvent: fn(),
    reconnect: fn(),
    dispose: fn(),
    connect: fn(),
    startPeriodicRefreshSession: fn(),
    refreshSession: fn(),
    getBearerToken: fn(),
}

impl SDK {
    pub fn new(&mut self, conf: Config) -> Self {
        // todo: Setup logging env_logger
        
        Self {
            csdsClient = new CSDSClient(conf),
            config,
            refreshSessionInterval: conf.refreshSessionInterval || 60000 * 1000, // 10 min
            accountId: conf.accountId,
        }
    }

    fn registerRequests() {}

    fn connect(&mut self) {
        // 1. get CSDS entries

        handleCSDS("Connect#CSDS");
        // self.config.domains = handleCSDS('connect#csds');

        // 2. login

        // 3. init

        init();

        // listen(wsPath!(&self.config), |out| Transport {
        //     out,
        //     ping_timeout: None,
        //     expire_timeout: None,
        // })
        // .unwrap();
    }

    fn handleCSDS(&mut self, location: String) -> Result<()> {
        let domains = self.csdsClient.getAll(location)?;
        domains
    }

    fn handleLogin(domains, location: String) -> Result<()> {

    }

    fn init(config: Config) {

    }

    // gets a bearer token from agentVEP
    fn login(&mut self, conf: Config, domains: Domain) {
        assert_ne!(domains.agentVep, "", "Couldn't fetch domains");

        let conf = self.config;

        if conf.token {
            self.userId = conf.userId; // for internal use
            self.oldAgentId = format!({}.{}, conf.accountId, conf.userId); // for external use
            return Ok(());
        }

        let loginData = Config {
            domain: domains.agentVep,
            ..conf,
        }

        // external.login


    }
}

struct Domain {
    agentVep: String,
}