use std::sync::Arc;
use tokio::sync::Mutex;

use crate::icy::IcyMetadata;

#[derive(Debug, Clone, PartialEq)]
pub struct MacOsMediaState {
    pub playing: bool,
    pub station_name: String,
    pub track_title: String,
    pub track_artist: Option<String>,
    pub url: String,
    pub icy_metadata: Option<IcyMetadata>,
}

impl Default for MacOsMediaState {
    fn default() -> Self {
        Self {
            playing: false,
            station_name: String::new(),
            track_title: String::new(),
            track_artist: None,
            url: String::new(),
            icy_metadata: None,
        }
    }
}

pub struct MacOsMediaCenter {
    state: Arc<Mutex<MacOsMediaState>>,
}

impl MacOsMediaCenter {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(MacOsMediaState::default())),
        }
    }

    pub fn state(&self) -> Arc<Mutex<MacOsMediaState>> {
        self.state.clone()
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn update_now_playing(&self) {
        #[cfg(target_os = "macos")]
        {
            self.update_now_playing_impl();
        }
    }

    #[cfg(target_os = "macos")]
    fn update_now_playing_impl(&self) {
        use objc2::runtime::AnyObject;
        use objc2::{class, msg_send};
        use objc2_foundation::NSString;
        use objc2_media_player::MPNowPlayingInfoCenter;

        unsafe {
            let state = self.state.try_lock().unwrap();
            let info_center = MPNowPlayingInfoCenter::defaultCenter();

            let info: *mut AnyObject = msg_send![class!(NSMutableDictionary), dictionary];

            let title = if state.track_title.is_empty() {
                state.station_name.clone()
            } else {
                state.track_title.clone()
            };

            if !title.is_empty() {
                let title_ns = NSString::from_str(&title);
                let key = NSString::from_str("kMRMediaItemPropertyTitle");
                let _: () = msg_send![info, setObject: &*title_ns, forKey: &*key];
            }

            if let Some(ref artist) = state.track_artist {
                let artist_ns = NSString::from_str(artist);
                let key = NSString::from_str("kMRMediaItemPropertyArtist");
                let _: () = msg_send![info, setObject: &*artist_ns, forKey: &*key];
            }

            let duration_key = NSString::from_str("kMRMediaItemPropertyPlaybackDuration");
            let duration_val: *mut AnyObject = msg_send![class!(NSNumber), numberWithDouble: 0.0];
            let _: () = msg_send![info, setObject: duration_val, forKey: &*duration_key];

            let rate = if state.playing { 1.0 } else { 0.0 };
            let rate_key = NSString::from_str("kMPNowPlayingInfoPropertyPlaybackRate");
            let rate_val: *mut AnyObject = msg_send![class!(NSNumber), numberWithDouble: rate];
            let _: () = msg_send![info, setObject: rate_val, forKey: &*rate_key];

            let elapsed_key = NSString::from_str("kMPNowPlayingInfoPropertyElapsedPlaybackTime");
            let elapsed_val: *mut AnyObject = msg_send![class!(NSNumber), numberWithDouble: 0.0];
            let _: () = msg_send![info, setObject: elapsed_val, forKey: &*elapsed_key];

            let _: () = msg_send![&*info_center, setNowPlayingInfo: info];

            let playback_state: i64 = if state.playing { 1 } else { 0 };
            let _: () = msg_send![&*info_center, setPlaybackState: playback_state];
        }
    }
}

impl Default for MacOsMediaCenter {
    fn default() -> Self {
        Self::new()
    }
}
