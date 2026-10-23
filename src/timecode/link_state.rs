use rusty_link::{AblLink, SessionState};

pub struct LinkState {
    pub link: AblLink,
    pub session_state: SessionState,
    pub quantum: f64,
}

impl LinkState {
    pub fn new() -> Self {
        Self {
            link: AblLink::new(120.),
            session_state: SessionState::new(),
            quantum: 4.,
        }
    }

    pub fn capture_app_state(&mut self) {
        self.link.capture_app_session_state(&mut self.session_state);
    }
}

impl Default for LinkState {
    fn default() -> Self {
        Self::new()
    }
}
