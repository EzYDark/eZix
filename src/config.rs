use crate::SystemManager;
use crate::modules;
use crate::modules::firewall::FirewallConfig;
use crate::modules::shell::ShellConfig;
use crate::modules::xserver::XServerConfig;

#[derive(Default)]
pub struct SystemConfig {
    pub services: Services,
    pub networking: Networking,
    pub shell: ShellConfig,
}

#[derive(Default)]
pub struct Services {
    pub xserver: XServerConfig,
}

#[derive(Default)]
pub struct Networking {
    pub firewall: FirewallConfig,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ModuleId {
    XServer,
    Firewall,
    Shell,
}

pub fn build_system(config: SystemConfig) -> SystemManager<ModuleId> {
    let mut manager = SystemManager::new();

    if config.services.xserver.enable {
        let module = modules::xserver::build(config.services.xserver);
        manager.add_module(ModuleId::XServer, module);
    }

    if config.networking.firewall.enable {
        let module = modules::firewall::build(config.networking.firewall);
        manager.add_module(ModuleId::Firewall, module);
    }

    if config.shell.enable {
        let module = modules::shell::build(config.shell);
        manager.add_module(ModuleId::Shell, module);
    }

    manager
}
