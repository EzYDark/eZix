use ezix::SystemConfig;
use ezix::modules::firewall::Firewall;

fn main() {
    ezlog::logger::init();

    // let base_config = ezix!(Firewall {
    //     enabled: true,
    //     ports: vec![22, 80, 443],
    // });

    let sys_config = SystemConfig::new().with(Firewall {
        enabled: true,
        ports: vec![22, 80, 443],
        ..Default::default()
    });
    // .extend(base_config);

    match sys_config.apply() {
        Ok(_) => log::info!("System configuration applied successfully"),
        Err(e) => log::error!("Failed to apply system configuration:\n - {}", e),
    }
}
