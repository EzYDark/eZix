// Example module

use crate::Funcs;
use anyhow::Result;

#[derive(Default)]
pub struct ControlPanel {
    pub enabled: bool,
    pub install: bool,
    pub package: &'static str,
    pub sandbox: bool,
}

impl Funcs for ControlPanel {
    fn id(&self) -> &'static str {
        "Control Panel"
    }
    fn enable(&self) -> Result<()> {
        log::debug!("Enabling 'Control Panel' module...");
        Ok(())
    }
    fn disable(&self) -> Result<()> {
        log::debug!("Disabling 'Control Panel' module...");
        Ok(())
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}
