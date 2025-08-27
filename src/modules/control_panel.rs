use crate::Funcs;

pub struct ControlPanel {
    pub enabled: bool,
}

impl Default for ControlPanel {
    fn default() -> Self {
        Self { enabled: false }
    }
}

impl Funcs for ControlPanel {
    fn name(&self) -> &'static str {
        "control_panel"
    }
    fn enable(&self) {
        log::debug!("Control panel enabled");
    }
    fn disable(&self) {
        log::debug!("Control panel disabled");
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}
