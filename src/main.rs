use ezix::ezix;
use ezix::modules::firewall::Firewall;
use ezix::modules::xserver::XServer;

fn main() {
    ezlog::logger::init();

    let sys_config = ezix!(
        Firewall {
            enabled: true,
            ports: vec![22, 80, 443],
        },
        XServer {
            enabled: true,
            window_manager: "i3"
        }
    );

    sys_config.apply();
}
