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
    fn id(&self) -> &'static str {
        "control_panel"
    }
    fn enable(&self) -> Result<(), anyhow::Error> {
        log::debug!("Control panel enabled");
        Ok(())
    }
    fn disable(&self) -> Result<(), anyhow::Error> {
        log::debug!("Control panel disabled");
        Ok(())
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}
