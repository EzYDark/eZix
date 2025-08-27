use ezix::SystemConfig;
use ezix::ezix;
use ezix::modules::control_panel::ControlPanel;
use ezix::modules::firewall::Firewall;
use ezix::modules::xserver::XServer;

fn main() {
    ezlog::logger::init();

    let base_config = ezix!(Firewall {
        enabled: true,
        ports: vec![22, 80, 443],
    });

    let base_config2 = ezix!(XServer {
        enabled: false,
        window_manager: "i3",
    });

    let sys_config = SystemConfig::new()
        .combine(base_config2)
        .with(XServer {
            enabled: true,
            window_manager: "i3",
        })
        .combine(base_config);

    sys_config.apply();
}
