use crate::Funcs;

pub struct Firewall {
    pub enabled: bool,
    pub ports: Vec<u16>,
}

impl Default for Firewall {
    fn default() -> Self {
        Self {
            enabled: false,
            ports: Vec::new(),
        }
    }
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
