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
        "firewall"
    }
    fn enable(&self) -> Result<(), anyhow::Error> {
        log::debug!("Firewall enabled");
        Ok(())
    }
    fn disable(&self) -> Result<(), anyhow::Error> {
        log::debug!("Firewall disabled");
        Ok(())
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}
