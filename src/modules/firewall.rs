use crate::{Module, ModuleBuilder, State};

#[derive(Clone, Debug, Default)]
pub struct FirewallConfig {
    pub enable: bool,
    pub allowed_ports: Vec<u16>,
}

pub fn build(config: FirewallConfig) -> Module {
    ModuleBuilder::new()
        .set_enable_fn(move |s: &mut State| {
            log::info!(
                "Configuring firewall, allowing ports: {:?}",
                config.allowed_ports
            );
            // In a real implementation, this would execute firewall commands.
            s.enabled = true;
        })
        .build()
}
