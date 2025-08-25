use crate::Funcs;

pub struct XServer {
    pub enabled: bool,
    pub window_manager: &'static str,
}

impl Default for XServer {
    fn default() -> Self {
        Self {
            enabled: false,
            window_manager: "",
        }
    }
}

impl Funcs for XServer {
    fn name(&self) -> &'static str {
        "xserver"
    }
    fn enable(&self) {
        log::debug!("XServer enabled");
    }
    fn disable(&self) {
        log::debug!("XServer disabled");
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}
