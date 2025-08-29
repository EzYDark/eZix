// Example module

use crate::Funcs;

#[derive(Default)]
pub struct Firewall {
    pub enabled: bool,
    pub install: bool,
    pub package: &'static str,
    pub sandbox: bool,

    pub ports: Vec<u16>,
}

impl Funcs for Firewall {
    fn id(&self) -> &'static str {
        "Firewall"
    }
    fn enable(&self) -> Result<(), anyhow::Error> {
        log::debug!("Enabling 'Firewall' module...");
        Ok(())
    }
    fn disable(&self) -> Result<(), anyhow::Error> {
        log::debug!("Disabling 'Firewall' module...");
        Ok(())
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}
