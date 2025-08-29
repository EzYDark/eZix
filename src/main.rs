use ezix::SystemConfig;
use ezix::modules::firewall::Firewall;
use ezix::modules::zen_browser::{Policies, PrefValue, Prefs, ZenBrowser};

fn main() {
    ezlog::logger::init();

    // let base_config = ezix!(Firewall {
    //     enabled: true,
    //     ports: vec![22, 80, 443],
    // });

    let sys_config = SystemConfig::new()
        .with(Firewall {
            enabled: true,
            ports: vec![22, 80, 443],
            ..Default::default()
        })
        .with(ZenBrowser {
            enabled: true,
            policies: Policies {
                disable_app_update: Some(true),
                background_app_update: Some(false),
                dont_check_default_browser: Some(true),
                no_default_bookmarks: Some(true),
                extra: serde_json::Map::new(),
            },
            prefs: Prefs {
                browser_startup_homepage: Some("https://example.org"),
                browser_startup_page: Some(3),
                extra: vec![("privacy.donottrackheader.enabled", PrefValue::Bool(true))],
                ..Default::default()
            },
            ..Default::default()
        });
    // .extend(base_config);

    match sys_config.apply() {
        Ok(_) => log::info!("System configuration applied successfully"),
        Err(e) => log::error!("Failed to apply system configuration:\n - {}", e),
    }
}
