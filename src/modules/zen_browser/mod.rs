use crate::Funcs;

mod paths;
mod util;

use anyhow::Context;
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
        "zen_browser"
    }

    fn enable(&self) -> Result<(), anyhow::Error> {
        log::debug!("Configuring Zen Browser");

        util::ensure_parent_dir(paths::POLICY_PATH)?;
        util::ensure_parent_dir(paths::AUTOCONFIG_JS)?;
        util::ensure_parent_dir(paths::MOZILLA_CFG)?;

        // 1) policies.json
        let policies_value = policies_to_json(&self.policies);
        let policies_pretty =
            serde_json::to_string_pretty(&policies_value).context("serialize policies.json")?;
        fs::write(paths::POLICY_PATH, policies_pretty)
            .with_context(|| format!("write {}", paths::POLICY_PATH))?;
        log::debug!("policies.json -> '{}'", paths::POLICY_PATH);

        // 2) autoconfig.js
        let autoconfig = util::render_autoconfig_js();
        fs::write(paths::AUTOCONFIG_JS, autoconfig)
            .with_context(|| format!("write {}", paths::AUTOCONFIG_JS))?;
        log::debug!("autoconfig.js -> '{}'", paths::AUTOCONFIG_JS);

        // 3) mozilla.cfg
        let cfg = render_cfg_from_prefs(&self.prefs);
        fs::write(paths::MOZILLA_CFG, cfg)
            .with_context(|| format!("write {}", paths::MOZILLA_CFG))?;
        log::debug!("mozilla.cfg -> '{}'", paths::MOZILLA_CFG);

        Ok(())
    }

    fn disable(&self) -> Result<(), anyhow::Error> {
        util::remove_file_silent(paths::POLICY_PATH)?;
        util::remove_file_silent(paths::AUTOCONFIG_JS)?;
        util::remove_file_silent(paths::MOZILLA_CFG)?;
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
