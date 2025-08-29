use crate::Funcs;

mod paths;
mod util;

use anyhow::{Context, Result};
use serde_json::{Map, Value, json};
use std::fs;

pub use util::PrefValue;

/// Declarative Zen Browser management:
/// - policies.json from `ZenBrowser.policies`
/// - autoconfig.js to activate mozilla.cfg
/// - mozilla.cfg from `ZenBrowser.prefs`
#[derive(Default)]
pub struct ZenBrowser {
    pub enabled: bool,
    pub install: bool,
    pub package: &'static str,
    pub sandbox: bool,

    pub policies: Policies,
    pub prefs: Prefs,
}

#[derive(Default)]
pub struct Policies {
    pub disable_app_update: Option<bool>,
    pub background_app_update: Option<bool>,
    pub dont_check_default_browser: Option<bool>,
    pub no_default_bookmarks: Option<bool>,

    /// Anything not covered by the typed fields lands here.
    pub extra: Map<String, Value>,
}

#[derive(Default)]
pub struct Prefs {
    // examples, extend as needed
    pub app_update_auto: Option<bool>,    // app.update.auto
    pub app_update_enabled: Option<bool>, // app.update.enabled
    pub browser_shell_check_default_browser: Option<bool>, // browser.shell.checkDefaultBrowser
    pub browser_startup_homepage: Option<&'static str>, // browser.startup.homepage
    pub browser_startup_page: Option<i64>, // browser.startup.page

    /// Additional arbitrary prefs
    pub extra: Vec<(&'static str, PrefValue)>,
}

impl Funcs for ZenBrowser {
    fn id(&self) -> &'static str {
        "Zen Browser"
    }

    fn enable(&self) -> Result<()> {
        log::debug!("Enabling 'Zen Browser' module...");

        if let Err(e) = util::ensure_parent_dir(paths::POLICY_PATH) {
            return Err(anyhow::anyhow!(
                "Failed to create parent directory for 'policies.json':\n - {}",
                e
            ));
        }
        if let Err(e) = util::ensure_parent_dir(paths::AUTOCONFIG_JS) {
            return Err(anyhow::anyhow!(
                "Failed to create parent directory for 'autoconfig.js':\n - {}",
                e
            ));
        }
        if let Err(e) = util::ensure_parent_dir(paths::MOZILLA_CFG) {
            return Err(anyhow::anyhow!(
                "Failed to create parent directory for 'mozilla.cfg':\n - {}",
                e
            ));
        }

        // 1) policies.json
        let policies_value = policies_to_json(&self.policies);
        match serde_json::to_string_pretty(&policies_value) {
            Ok(policies_pretty) => match fs::write(paths::POLICY_PATH, policies_pretty) {
                Ok(_) => log::debug!("'policies.json' written to '{}'", paths::POLICY_PATH),
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Failed to write 'policies.json':\n - {}",
                        e
                    ));
                }
            },
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to serialize into 'policies.json':\n - {}",
                    e
                ));
            }
        }

        // 2) autoconfig.js
        let autoconfig = util::render_autoconfig_js();
        match fs::write(paths::AUTOCONFIG_JS, autoconfig) {
            Ok(_) => log::debug!("'autoconfig.js' written to '{}'", paths::AUTOCONFIG_JS),
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to write 'autoconfig.js':\n - {}",
                    e
                ));
            }
        }

        // 3) mozilla.cfg
        let cfg = render_cfg_from_prefs(&self.prefs);
        match fs::write(paths::MOZILLA_CFG, cfg) {
            Ok(_) => log::debug!("'mozilla.cfg' written to '{}'", paths::MOZILLA_CFG),
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to write 'mozilla.cfg':\n - {}", e));
            }
        }

        Ok(())
    }

    fn disable(&self) -> Result<()> {
        match util::remove_file_silent(paths::POLICY_PATH) {
            Ok(_) => log::debug!("'policies.json' removed from '{}'", paths::POLICY_PATH),
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to remove 'policies.json':\n - {}",
                    e
                ));
            }
        }
        match util::remove_file_silent(paths::AUTOCONFIG_JS) {
            Ok(_) => log::debug!("'autoconfig.js' removed from '{}'", paths::AUTOCONFIG_JS),
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to remove 'autoconfig.js':\n - {}",
                    e
                ));
            }
        }
        match util::remove_file_silent(paths::MOZILLA_CFG) {
            Ok(_) => log::debug!("'mozilla.cfg' removed from '{}'", paths::MOZILLA_CFG),
            Err(e) => return Err(anyhow::anyhow!("Failed to remove 'mozilla.cfg':\n - {}", e)),
        }
        log::debug!("Zen Browser configuration removed");
        Ok(())
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

fn policies_to_json(p: &Policies) -> Value {
    let mut m = Map::new();
    if let Some(v) = p.disable_app_update {
        m.insert("DisableAppUpdate".into(), json!(v));
    }
    if let Some(v) = p.background_app_update {
        m.insert("BackgroundAppUpdate".into(), json!(v));
    }
    if let Some(v) = p.dont_check_default_browser {
        m.insert("DontCheckDefaultBrowser".into(), json!(v));
    }
    if let Some(v) = p.no_default_bookmarks {
        m.insert("NoDefaultBookmarks".into(), json!(v));
    }
    // user extras win
    for (k, v) in &p.extra {
        m.insert(k.clone(), v.clone());
    }
    json!({ "policies": m })
}

fn render_cfg_from_prefs(p: &Prefs) -> String {
    use PrefValue::*;
    let mut out: Vec<(&'static str, PrefValue)> = Vec::new();

    if let Some(v) = p.app_update_auto {
        out.push(("app.update.auto", Bool(v)));
    }
    if let Some(v) = p.app_update_enabled {
        out.push(("app.update.enabled", Bool(v)));
    }
    if let Some(v) = p.browser_shell_check_default_browser {
        out.push(("browser.shell.checkDefaultBrowser", Bool(v)));
    }
    if let Some(v) = p.browser_startup_homepage {
        out.push(("browser.startup.homepage", Str(v)));
    }
    if let Some(v) = p.browser_startup_page {
        out.push(("browser.startup.page", Int(v)));
    }

    out.extend_from_slice(&p.extra);

    util::render_mozilla_cfg(&out)
}
