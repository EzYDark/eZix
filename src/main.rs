use ezix::SystemConfig;
use ezix::modules::xserver::XServer;

fn main() {
    ezlog::logger::init();

    // let base_config = ezix!(Firewall {
    //     enabled: true,
    //     ports: vec![22, 80, 443],
    // });

    let sys_config = SystemConfig::new().with(XServer {
        enabled: true,
        window_manager: "i3",
    });
    // .extend(base_config);

    let _ = sys_config.apply();
}
