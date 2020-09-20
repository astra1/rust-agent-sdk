//! Minimal Agent SDK implemenation
//! Provides an async connect and methods for issuing the supported commands.
//! Uses Mini-Redis as alma mater

use shared::structs::Config;
use std::time::Duration;
use tokio::net::{TcpStream, ToSocketAddrs};
/// Establish connection with AgentSDK websocket server;
///
/// Requests are issued using the various methods of `Client`.
#[allow(dead_code)]
pub struct SDK {
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
    fn new(conf: Config) {
        assert_ne!(conf.accountId, 0);
        assert_ne!(conf.requestTimeout, 0,);
        assert_ne!(conf.errorCheckInterval, 0,);
        assert_ne!(conf.apiVersion, 0);
        assert_ne!(conf.refreshSessionInterval, 0);
    }
}

/// A client that has entered pub/sub mode.
///
/// Once clients subscribe to a channel, they may only perform pub/sub related
/// commands. The `Client` type is transitioned to a `Subscriber` type in order
/// to prevent non-pub/sub methods from being called.
#[allow(dead_code)]
pub struct Subscriber {
    /// The subscribed client.
    sdk: SDK,

    /// The set of channels to which the `Subscriber` is currently subscribed.
    subscribed_channels: Vec<String>,
}

/// A message received on a subscribed channel.
#[derive(Debug, Clone)]
pub struct Message {
    pub channel: String,
    pub content: Bytes,
}

/// Establish a connection with the Redis server located at `addr`.
///
/// `addr` may be any type that can be asynchronously converted to a
/// `SocketAddr`. This includes `SocketAddr` and strings. The `ToSocketAddrs`
/// trait is the Tokio version and not the `std` version.
///
/// # Examples
///
/// ```no_run
/// use mini_redis::client;
///
/// #[tokio::main]
/// async fn main() {
///     let client = match client::connect("localhost:6379").await {
///         Ok(client) => client,
///         Err(_) => panic!("failed to establish connection"),
///     };
/// # drop(client);
/// }
/// ```
#[allow(dead_code)]
pub async fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<SDK> {
    // The `addr` argument is passed directly to `TcpStream::connect`. This
    // performs any asynchronous DNS lookup and attempts to establish the TCP
    // connection. An error at either step returns an error, which is then
    // bubbled up to the caller of `mini_redis` connect.
    let socket = TcpStream::connect(addr).await?;

    // Initialize the connection state. This allocates read/write buffers to
    // perform redis protocol frame parsing.
    let connection = Connection::new(socket);

    Ok(SDK { connection })
}

impl SDK {
    fn new(conf: Config) {}
}
