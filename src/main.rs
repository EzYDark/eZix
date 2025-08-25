use ezix::config::{Networking, Services, SystemConfig, build_system};
use ezix::modules::firewall::FirewallConfig;
use ezix::modules::shell::{PackageManager, ShellConfig};
use ezix::modules::xserver::XServerConfig;

fn main() -> Result<(), anyhow::Error> {
    ezix::logger::init();

    let my_system = SystemConfig {
        services: Services {
            xserver: XServerConfig {
                enable: true,
                layout: "us",
                ..Default::default()
            },
        },
        networking: Networking {
            firewall: FirewallConfig {
                enable: true,
                allowed_ports: vec![80, 443],
                ..Default::default()
            },
        },
        shell: ShellConfig {
            enable: true,
            package_manager: PackageManager::Scoop,
            packages: vec!["git", "neovim"],
            ..Default::default()
        },
        ..Default::default()
    };

    let mut manager = build_system(my_system);
    manager.apply();

    Ok(())
}
