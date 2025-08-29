use ezix::SystemConfig;
use ezix::modules::firewall::Firewall;
use ezix::modules::zen_browser::{Policies, PrefValue, Prefs, ZenBrowser};

fn main() {
    ezlog::logger::init();

    let sys_config = SystemConfig::new()
        .with(Firewall {
            enabled: true,
            ports: vec![22, 80, 443],
            ..Default::default()
        })
        .with(ZenBrowser {
            enabled: true,
            prefs: Prefs {
                restore_prev_session: Some(true),
                continue_where_you_left_off: Some(true),
                extra: vec![("privacy.donottrackheader.enabled", PrefValue::Bool(true))],
                ..Default::default()
            },
            ..Default::default()
        });

    if let Err(e) = sys_config.apply() {
        // Pretty-print full error chain
        log::error!("Failed to apply system configuration:\n{:#}", e);
    } else {
        log::info!("System configuration applied successfully");
    }
}
