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

type VoidCallback = Arc<Mutex<Option<Box<dyn Fn() + Send + Sync>>>>;

pub struct MacOsMediaCenter {
    state: Arc<Mutex<MacOsMediaState>>,
    play_pause_callback: VoidCallback,
    next_callback: VoidCallback,
    prev_callback: VoidCallback,
}

impl MacOsMediaCenter {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(MacOsMediaState::default())),
            play_pause_callback: Arc::new(Mutex::new(None)),
            next_callback: Arc::new(Mutex::new(None)),
            prev_callback: Arc::new(Mutex::new(None)),
        }
    }

    pub fn state(&self) -> Arc<Mutex<MacOsMediaState>> {
        self.state.clone()
    }

    pub fn on_play_pause<F: Fn() + Send + Sync + 'static>(&self, callback: F) {
        if let Ok(mut cb) = self.play_pause_callback.try_lock() {
            *cb = Some(Box::new(callback));
        }
    }

    pub fn on_next<F: Fn() + Send + Sync + 'static>(&self, callback: F) {
        if let Ok(mut cb) = self.next_callback.try_lock() {
            *cb = Some(Box::new(callback));
        }
    }

    pub fn on_previous<F: Fn() + Send + Sync + 'static>(&self, callback: F) {
        if let Ok(mut cb) = self.prev_callback.try_lock() {
            *cb = Some(Box::new(callback));
        }
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
        use cocoa::base::{id, nil};
        use cocoa::foundation::NSString;
        use objc::{class, msg_send, sel, sel_impl};

        unsafe {
            let state = self.state.try_lock().unwrap();
            let info_center: id = msg_send![class!(MPNowPlayingInfoCenter), defaultCenter];

            let info: id = msg_send![class!(NSMutableDictionary), dictionary];

            let title = if state.track_title.is_empty() {
                state.station_name.clone()
            } else {
                state.track_title.clone()
            };

            if !title.is_empty() {
                let title_ns = NSString::alloc(nil).init_str(&title);
                let key = NSString::alloc(nil).init_str("kMRMediaItemPropertyTitle");
                let _: () = msg_send![info, setObject: title_ns forKey: key];
            }

            if let Some(ref artist) = state.track_artist {
                let artist_ns = NSString::alloc(nil).init_str(artist);
                let key = NSString::alloc(nil).init_str("kMRMediaItemPropertyArtist");
                let _: () = msg_send![info, setObject: artist_ns forKey: key];
            }

            let duration_key =
                NSString::alloc(nil).init_str("kMRMediaItemPropertyPlaybackDuration");
            let duration_val: id = msg_send![class!(NSNumber), numberWithDouble: 0.0];
            let _: () = msg_send![info, setObject: duration_val forKey: duration_key];

            let rate: f64 = if state.playing { 1.0 } else { 0.0 };
            let rate_key = NSString::alloc(nil).init_str("kMPNowPlayingInfoPropertyPlaybackRate");
            let rate_val: id = msg_send![class!(NSNumber), numberWithDouble: rate];
            let _: () = msg_send![info, setObject: rate_val forKey: rate_key];

            let elapsed_key =
                NSString::alloc(nil).init_str("kMPNowPlayingInfoPropertyElapsedPlaybackTime");
            let elapsed_val: id = msg_send![class!(NSNumber), numberWithDouble: 0.0];
            let _: () = msg_send![info, setObject: elapsed_val forKey: elapsed_key];

            let _: () = msg_send![info_center, setNowPlayingInfo: info];

            let playback_state: cocoa::foundation::NSInteger = if state.playing { 1 } else { 0 };
            let _: () = msg_send![info_center, setPlaybackState: playback_state];
        }
    }
}

impl Default for MacOsMediaCenter {
    fn default() -> Self {
        Self::new()
    }
}
