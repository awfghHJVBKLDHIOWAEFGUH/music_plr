use std::time::Duration;

use super::{warn, MusicPlayer};
use crate::{
    app::{pane::PaneId, Pane, SplitNode},
    data::{session::SessionState, JsonStore},
};

const SESSION_FLUSH_MIN_INTERVAL: Duration = Duration::from_secs(1);

impl MusicPlayer {
    pub fn save_session(&mut self) {
        self.session_dirty = true;
    }

    /// Persist the session, but at most once per `SESSION_FLUSH_MIN_INTERVAL`
    /// so that the frequent `save_session()` callers (volume, seek, drag,
    /// navigation) don't rewrite `session.json` on every tick.
    pub fn flush_session(&mut self) {
        if !self.session_dirty {
            return;
        }
        let now = std::time::Instant::now();
        if now.duration_since(self.last_session_flush) < SESSION_FLUSH_MIN_INTERVAL {
            return;
        }
        self.last_session_flush = now;
        self.session.save();
        self.session_dirty = false;
    }

    pub fn restore_session(&mut self) {
        self.session = SessionState::load_migrated();
        if self.session.panes.is_empty() {
            self.session.panes.insert(0, Pane::new(0));
        }
        // Drop panes the tree doesn't reference and fall back to a single
        // leaf when the saved tree is inconsistent.
        let mut leaves = Vec::new();
        self.session.root.leaves(&mut leaves);
        let valid: Vec<PaneId> = leaves
            .into_iter()
            .filter(|id| self.session.panes.contains_key(id))
            .collect();
        if valid.is_empty() {
            let id = *self.session.panes.keys().min().unwrap_or(&0);
            self.session.root = SplitNode::Leaf(id);
            self.session.focused = id;
        } else {
            self.session.panes.retain(|id, _| valid.contains(id));
            self.session.focused = if valid.contains(&self.session.focused) {
                self.session.focused
            } else {
                valid[0]
            };
        }
        self.next_pane_id = self.session.panes.keys().max().map_or(0, |m| m + 1).max(1);
        self.audio.set_volume(self.session.volume);
        for pane in self.pane_ids() {
            self.sync_downloads_view(pane);
        }
    }

    pub fn notify(&mut self, msg: impl Into<std::borrow::Cow<'static, str>>) {
        self.notification = Some(crate::app::Toast {
            message: msg.into(),
            until: std::time::Instant::now() + crate::app::update::tick::NOTIFICATION_DURATION,
            is_error: false,
        });
    }

    pub fn notify_for(
        &mut self,
        msg: impl Into<std::borrow::Cow<'static, str>>,
        duration: std::time::Duration,
    ) {
        self.notification = Some(crate::app::Toast {
            message: msg.into(),
            until: std::time::Instant::now() + duration,
            is_error: false,
        });
    }

    /// Surface a `YouTube` player-client race event as a toast, using the
    /// current language. The "none available" case is an error toast, but
    /// playback/download still falls back to yt-dlp's defaults.
    pub fn notify_client_event(&mut self, event: crate::providers::ClientEvent) {
        match event {
            crate::providers::ClientEvent::Resolving => {
                self.notify(self.strings.resolving_player_client);
            }
            crate::providers::ClientEvent::Resolved(client) => {
                self.notify((self.strings.resolved_player_client)(&client));
            }
            crate::providers::ClientEvent::Unavailable => {
                self.notify_error(self.strings.player_client_unavailable);
            }
        }
    }

    pub fn notify_error(&mut self, message: impl Into<std::borrow::Cow<'static, str>>) {
        let message = message.into();
        warn!("Backend error: {message}");
        self.notification = Some(crate::app::Toast {
            message,
            until: std::time::Instant::now() + crate::app::update::tick::NOTIFICATION_DURATION,
            is_error: true,
        });
    }
}
