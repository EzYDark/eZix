// Example module

use crate::Funcs;

#[derive(Default)]
pub struct ControlPanel {
    pub enabled: bool,
    pub install: bool,
    pub package: &'static str,
    pub sandbox: bool,
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
