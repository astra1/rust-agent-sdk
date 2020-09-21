//! Minimal Agent SDK implemenation
//! Provides an async connect and methods for issuing the supported commands.
//! Uses Mini-Redis as alma mater

use crate::structs::Config;
use crate::external_services;
use std::time::Duration;
use tokio::net::{TcpStream, ToSocketAddrs};
use crate::transport::{Transport};
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
    agentId: u32,

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
        self = Self {
            csdsClient = new CSDSClient(conf),
            config,
            refreshSessionInterval: conf.refreshSessionInterval || 60000 * 1000, // 10 min
            accountId: conf.accountId,
        }
        self.connect(&self, conf);
    }

    fn registerRequests() {}

    fn connect(&mut self) {
        // 1. get CSDS entries

        let domains = handleCSDS("Connect#CSDS")?;

        // 2. login
        let (loginDomains, token) = login(&domains, "Connect#Login")?;

        // 3. init

        init(Config {
            domain: loginDomains.facadeMsg,
            token,
            ..self.config
        });

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


    fn init(&mut self, config: Config) {
        self.requestTimeout = config.requestTimeout || 10_000;
        self.requestTimeout = config.requestTimeout || 10_000;
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

        let data = external_services::login(loginData)?;

        match data {
            Ok(data, cookies) => {
                self.agentId = data.config.userPid;
                self.userId = data.config.userId;
                if self.userId > 0 {
                    self.oldAgentId = format!("{}.{}", conf.accountId, self.userId);
                } else {
                    return Err("Invalid login state, userId is undefined");
                }

                if data.csrf {
                    self.jar = cookies;
                    self.csrf = data.csrf;
                    self.token = data.bearer;
                    // startPeriodicRefreshSession();
                }

                Ok(data.bearer, domains);
            }
            Err(e) => println!("error external login {:?}", e),

        }

    }
}

struct Domain {
    agentVep: String,
}

macro_rules! wsPath {
    (config: Config) => {
        format!(
            "wss://{domain}/ws_api/account/{accountId}/messaging/brand/{token}?v=${apiVersion}",
            domain = config.domain,
            accountId = config.accountId,
            token = config.token,
            apiVersion = config.apiVersion
        )
    };
}