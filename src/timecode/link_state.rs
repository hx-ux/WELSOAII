use rusty_link::{AblLink, SessionState};
use std::sync::Mutex;

pub struct LinkState {
    pub link: AblLink,
    pub session_state: Mutex<SessionState>,
    pub quantum: f64,
}

impl LinkState {
    pub fn new() -> Self {
        Self {
            link: AblLink::new(120.),
            session_state: Mutex::new(SessionState::new()),
            quantum: 4.,
        }
    }

    pub fn capture_app_state(&mut self) {
        if let Ok(session_state) = self.session_state.get_mut() {
            self.link.capture_app_session_state(session_state);
        }
    }
}

impl Default for LinkState {
    fn default() -> Self {
        Self::new()
    }
}
