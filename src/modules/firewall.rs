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
    fn name(&self) -> &'static str {
        "firewall"
    }
    fn enable(&self) {
        log::debug!("Firewall enabled");
    }
    fn disable(&self) {
        log::debug!("Firewall disabled");
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}
