#[allow(dead_code)]
pub enum Events {
    Connected,
    Closed,
    Notification,
}

#[allow(dead_code)]
pub enum Kinds {
    Request,
    Response,
    Notification,
}

// const BROWSERS: &'static [&'static str] = &["firefox", "chrome"];

pub const REQUESTS: &'static [&'static str] = &[
    ".GetClock",
    ".ams.cm.AgentRequestConversation",
    ".ams.aam.SubscribeExConversations",
    ".ams.aam.UnsubscribeExConversations",
    ".ams.cm.UpdateConversationField",
    ".ams.ms.PublishEvent",
    ".ams.routing.UpdateRingState",
    ".ams.routing.SubscribeRoutingTasks",
    ".ams.routing.UpdateRoutingTaskSubscription",
    ".ams.userprofile.GetUserProfile",
    ".ams.routing.SetAgentState",
    ".ams.routing.SubscribeAgentsState",
    "ms.SubscribeMessagingEvents",
    ".ams.ms.GenerateURLForDownloadFile",
    ".ams.ms.GenerateURLForUploadFile",
];
