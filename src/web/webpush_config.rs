use serde::Serialize;
use serde_json::Value;

use super::webpush_fcm_options::{WebpushFcmOptions, WebpushFcmOptionsInternal};

#[derive(Serialize, Debug)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#webpushconfig
pub(crate) struct WebpushConfigInternal<'m> {
    /// HTTP headers defined in webpush protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<&'m Value>,

    /// Arbitrary key/value payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<&'m Value>,

    /// Web Notification options as a JSON object.
    /// Struct format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Struct
    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<&'m Value>,

    /// Options for features provided by the FCM SDK for Web.
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<WebpushFcmOptionsInternal<'m>>,
}

#[derive(Debug, Default)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#webpushconfig
pub struct WebpushConfig {
    /// HTTP headers defined in webpush protocol.
    pub headers: Option<Value>,

    /// Arbitrary key/value payload.
    pub data: Option<Value>,

    /// Web Notification options as a JSON object.
    /// Struct format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Struct
    pub notification: Option<Value>,

    /// Options for features provided by the FCM SDK for Web.
    pub fcm_options: Option<WebpushFcmOptions>,
}

impl WebpushConfig {
    pub(crate) fn finalize(&self) -> WebpushConfigInternal {
        WebpushConfigInternal {
            headers: self.headers.as_ref(),
            data: self.data.as_ref(),
            notification: self.notification.as_ref(),
            fcm_options: self.fcm_options.as_ref().map(|fcm_options| fcm_options.finalize()),
        }
    }
}
