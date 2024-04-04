use serde::Serialize;

use super::{
    light_settings::{LightSettings, LightSettingsInternal},
    notification_priority::NotificationPriority,
    visibility::Visibility,
};

#[derive(Serialize, Debug)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#androidnotification
pub(crate) struct AndroidNotificationInternal<'m> {
    /// The notification's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'m str>,

    /// The notification's body text.
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'m str>,

    /// The notification's icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<&'m str>,

    /// The notification's icon color, expressed in #rrggbb format.
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<&'m str>,

    /// The sound to play when the device receives the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<&'m str>,

    /// Identifier used to replace existing notifications in the notification drawer.
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<&'m str>,

    /// The action associated with a user click on the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    click_action: Option<&'m str>,

    /// The key to the body string in the app's string resources to use to localize the body text to the user's
    /// current localization.
    #[serde(skip_serializing_if = "Option::is_none")]
    body_loc_key: Option<&'m str>,

    /// Variable string values to be used in place of the format specifiers in body_loc_key to use to localize the
    /// body text to the user's current localization.
    #[serde(skip_serializing_if = "Option::is_none")]
    body_loc_args: Option<&'m [String]>,

    /// The key to the title string in the app's string resources to use to localize the title text to the user's
    /// current localization.
    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_key: Option<&'m str>,

    /// Variable string values to be used in place of the format specifiers in title_loc_key to use to localize the
    /// title text to the user's current localization.
    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_args: Option<&'m [String]>,

    /// The notification's channel id (new in Android O).
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<&'m str>,

    /// Sets the "ticker" text, which is sent to accessibility services.
    #[serde(skip_serializing_if = "Option::is_none")]
    ticker: Option<&'m str>,

    /// When set to false or unset, the notification is automatically dismissed when the user clicks it in the panel.
    #[serde(skip_serializing_if = "Option::is_none")]
    sticky: Option<bool>,

    /// Set the time that the event in the notification occurred. Notifications in the panel are sorted by this time.
    /// Timestamp format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    event_time: Option<&'m str>,

    /// Set whether or not this notification is relevant only to the current device.
    #[serde(skip_serializing_if = "Option::is_none")]
    local_only: Option<bool>,

    /// Set the relative priority for this notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_priority: Option<NotificationPriority>,

    /// If set to true, use the Android framework's default sound for the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    default_sound: Option<bool>,

    /// If set to true, use the Android framework's default vibrate pattern for the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    default_vibrate_timings: Option<bool>,

    /// If set to true, use the Android framework's default LED light settings for the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    default_light_settings: Option<bool>,

    /// Set the vibration pattern to use
    /// Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    #[serde(skip_serializing_if = "Option::is_none")]
    vibrate_timings: Option<&'m [String]>,

    /// Set the Notification.visibility of the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<Visibility>,

    /// Sets the number of items this notification represents.
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_count: Option<i32>,

    /// Settings to control the notification's LED blinking rate and color if LED is available on the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    light_settings: Option<LightSettingsInternal<'m>>,

    /// Contains the URL of an image that is going to be displayed in a notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<&'m str>,
}

#[derive(Debug, Default)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#androidnotification
pub struct AndroidNotification {
    /// The notification's title.
    pub title: Option<String>,

    /// The notification's body text.
    pub body: Option<String>,

    /// The notification's icon.
    pub icon: Option<String>,

    /// The notification's icon color, expressed in #rrggbb format.
    pub color: Option<String>,

    /// The sound to play when the device receives the notification.
    pub sound: Option<String>,

    /// Identifier used to replace existing notifications in the notification drawer.
    pub tag: Option<String>,

    /// The action associated with a user click on the notification.
    pub click_action: Option<String>,

    /// The key to the body string in the app's string resources to use to localize the body text to the user's
    /// current localization.
    pub body_loc_key: Option<String>,

    /// Variable string values to be used in place of the format specifiers in body_loc_key to use to localize the
    /// body text to the user's current localization.
    pub body_loc_args: Option<Vec<String>>,

    /// The key to the title string in the app's string resources to use to localize the title text to the user's
    /// current localization.
    pub title_loc_key: Option<String>,

    /// Variable string values to be used in place of the format specifiers in title_loc_key to use to localize the
    /// title text to the user's current localization.
    pub title_loc_args: Option<Vec<String>>,

    /// The notification's channel id (new in Android O).
    pub channel_id: Option<String>,

    /// Sets the "ticker" text, which is sent to accessibility services.
    pub ticker: Option<String>,

    /// When set to false or unset, the notification is automatically dismissed when the user clicks it in the panel.
    pub sticky: Option<bool>,

    /// Set the time that the event in the notification occurred. Notifications in the panel are sorted by this time.
    /// Timestamp format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Timestamp
    pub event_time: Option<String>,

    /// Set whether or not this notification is relevant only to the current device.
    pub local_only: Option<bool>,

    /// Set the relative priority for this notification.
    pub notification_priority: Option<NotificationPriority>,

    /// If set to true, use the Android framework's default sound for the notification.
    pub default_sound: Option<bool>,

    /// If set to true, use the Android framework's default vibrate pattern for the notification.
    pub default_vibrate_timings: Option<bool>,

    /// If set to true, use the Android framework's default LED light settings for the notification.
    pub default_light_settings: Option<bool>,

    /// Set the vibration pattern to use
    /// Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    pub vibrate_timings: Option<Vec<String>>,

    /// Set the Notification.visibility of the notification.
    pub visibility: Option<Visibility>,

    /// Sets the number of items this notification represents.
    pub notification_count: Option<i32>,

    /// Settings to control the notification's LED blinking rate and color if LED is available on the device.
    pub light_settings: Option<LightSettings>,

    /// Contains the URL of an image that is going to be displayed in a notification.
    pub image: Option<String>,
}

impl AndroidNotification {
    pub(crate) fn finalize(&self) -> AndroidNotificationInternal {
        AndroidNotificationInternal {
            title: self.title.as_deref(),
            body: self.body.as_deref(),
            icon: self.icon.as_deref(),
            color: self.color.as_deref(),
            sound: self.sound.as_deref(),
            tag: self.tag.as_deref(),
            click_action: self.click_action.as_deref(),
            body_loc_key: self.body_loc_key.as_deref(),
            body_loc_args: self.body_loc_args.as_deref(),
            title_loc_key: self.title_loc_key.as_deref(),
            title_loc_args: self.title_loc_args.as_deref(),
            channel_id: self.channel_id.as_deref(),
            ticker: self.ticker.as_deref(),
            sticky: self.sticky,
            event_time: self.event_time.as_deref(),
            local_only: self.local_only,
            notification_priority: self.notification_priority,
            default_sound: self.default_sound,
            default_vibrate_timings: self.default_vibrate_timings,
            default_light_settings: self.default_light_settings,
            vibrate_timings: self.vibrate_timings.as_deref(),
            visibility: self.visibility,
            notification_count: self.notification_count,
            light_settings: self.light_settings.as_ref().map(|x| x.finalize()),
            image: self.image.as_deref(),
        }
    }
}
