use serde::Serialize;

#[allow(dead_code)]
#[derive(Clone, Copy, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#androidmessagepriority
pub enum AndroidMessagePriority {
    Normal,
    High,
}
