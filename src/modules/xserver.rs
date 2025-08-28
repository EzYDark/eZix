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
    fn id(&self) -> &'static str {
        "xserver"
    }
    fn enable(&self) -> Result<(), anyhow::Error> {
        log::debug!("XServer enabled");
        Ok(())
    }
    fn disable(&self) -> Result<(), anyhow::Error> {
        log::debug!("XServer disabled");
        Ok(())
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
}
