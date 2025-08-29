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
    /// General
    pub restore_prev_session: Option<bool>,
    pub continue_where_you_left_off: Option<bool>,
    pub ctrl_tab_tabs_cycling: Option<bool>,
    pub open_links_in_new_tab: Option<bool>,
    pub switch_to_media_immediately: Option<bool>,
    pub ask_before_closing_multiple_tabs: Option<bool>,
    pub tab_previews: Option<bool>,
    pub container_tabs: Option<bool>,

    /// Language and Appearance
    pub appearance: Option<AppearanceModePref>,
    pub font: Option<FontPref>,
    pub default_zoom: Option<ZoomPref>,
    pub language: Option<LanguagePref>,

    // Files and Applications
    pub downloads: Option<DownloadsPref>,
    pub what_todo_with_other_files: Option<WhatTodoWithOtherFilesPref>,
    pub play_drm_controlled_content: Option<bool>,

    // Updates
    pub updates: Option<UpdatesPref>,

    // Performance
    pub performance: Option<PerformancePref>,

    // Browsing
    pub autoscrolling: Option<bool>,
    pub smooth_scrolling: Option<bool>,
    pub touch_keyboard: Option<bool>,
    pub cursor_keys: Option<bool>,
    pub underline_links: Option<bool>,
    pub search_for_text_when_typing: Option<bool>,
    pub picture_in_picture_video_controls: Option<bool>,
    pub control_media_via_media_interfaces: Option<bool>,
    pub recommend_extensions: Option<bool>,
    pub recommend_features: Option<bool>,

    // Network
    pub network: Option<NetworkPref>,

    // Look and Feel
    pub browser_layout: Option<BrowserLayoutPref>,
    pub vertical_tabs: Option<VerticalTabsPref>,

    // Theme
    pub toolbar_popup_in_compact_mode: Option<bool>,
    pub themed_background_for_compact_toolbar: Option<bool>,
    pub themed_background_for_compact_sidebar: Option<bool>,

    // Glance
    pub glance: Option<bool>,

    // URL Bar
    pub url_bar_behaviour: Option<UrlBarBehaviourPref>,

    // Workspaces
    pub workspaces: Option<WorkspacesPref>,

    // Pinned tabs
    pub pinned_tabs: Option<PinnedTabsPref>,

    // Zen Mods
    pub zen_mods: Option<ZenModsPref>,

    // Home
    pub home: Option<HomePref>,

    // Search
    pub search: Option<SearchPref>,

    // Privacy and Security
    pub privacy_and_security: Option<PrivacyAndSecurityPref>,

    /// Additional arbitrary prefs
    pub extra: Vec<(&'static str, PrefValue)>,
}

#[derive(Debug, Clone, Copy)]
pub enum AppearanceModePref {
    System,
    Light,
    Dark,
}

pub struct FontPref {
    pub size: Option<i64>,
    pub default_font: Option<&'static str>,
}

pub struct ZoomPref {
    pub default_zoom: Option<i64>,
    pub zoom_text_only: Option<bool>,
}

pub struct LanguagePref {
    pub default_language: Option<&'static str>,
    pub check_spelling: Option<bool>,
}

pub struct DownloadsPref {
    pub default_path: Option<&'static str>,
    pub always_ask_where_to_save: Option<bool>,
}

pub enum WhatTodoWithOtherFilesPref {
    SaveFiles,
    Ask,
}

pub struct UpdatesPref {
    pub check_for_updates: Option<bool>,
    pub auto_update: Option<bool>,
    pub update_only_in_background: Option<bool>,
}

pub struct PerformancePref {
    pub use_recommended_settings: Option<bool>,
    pub use_hardware_acceleration: Option<bool>,
}

pub struct NetworkPref {
    pub proxy: Option<ProxyPref>,
}

pub struct ProxyPref {
    pub mode: Option<ProxyModePref>,
}

pub enum ProxyModePref {
    NoProxy,
    AutoDetect,
    SystemProxy,
    Manual,
}

pub enum BrowserLayoutPref {
    SingleToolbar,
    MultipleToolbars,
    CollapsedToolbar,
}

pub struct VerticalTabsPref {
    pub new_tab_button_on_tab_list: Option<bool>,
    pub new_tab_button_on_top: Option<bool>,
}

pub enum UrlBarBehaviourPref {
    Normal,
    FloatingWhenTyping,
    FloatingAlways,
}

pub struct WorkspacesPref {
    pub hide_container_indicator: Option<bool>,
    pub switch_to_default_container_workspace: Option<bool>,
}

pub struct PinnedTabsPref {
    pub restore_pinned_tabs_on_startup: Option<bool>,
    pub container_essentials: Option<bool>,
    pub close_tab_shortcut_behaviour: Option<CloseTabShortcutBehaviourPref>,
}

pub enum CloseTabShortcutBehaviourPref {
    ResetURLUnloadAndSwitchToNextTab,
    UnloadAndSwitchToNextTab,
    ResetURLAndSwitchToNextTab,
    SwitchToNextTab,
    ResetURL,
    CloseTab,
}

pub struct ZenModsPref {
    pub enabled: Option<bool>,
    pub auto_update_on_startup: Option<bool>,
}

pub struct HomePref {
    pub homepage_and_new_windows: Option<HomepageAndNewWindowsPref>,
    pub new_tabs: Option<NewTabsPref>,
    pub home_content: Option<Vec<HomeContentItem>>,
}

pub enum HomepageAndNewWindowsPref {
    FirefoxHome,
    CustomPage(&'static str),
    BlankPage,
}

pub enum NewTabsPref {
    FirefoxHome,
    BlankPage,
}

pub enum HomeContentItem {
    WebSearch,
    Shortcuts,
    SupportZenBrowser,
    SponsoredShortcuts,
    RecentActivity(RecentActivitySelection),
}

pub enum RecentActivitySelection {
    VisitedPages,
    Bookmarks,
    MostRecentDownload,
}

pub struct SearchPref {
    pub default_search_engine: Option<DefaultSearchEnginePref>,
    pub use_default_search_engine_in_private_windows: Option<bool>,
    pub default_search_engine_private_windows: Option<DefaultSearchEnginePref>,
    pub search_suggestions: Option<SearchSuggestionsPref>,
    pub address_bar: Option<AddressBarPref>,
}

pub enum DefaultSearchEnginePref {
    Google,
    Bing,
    DuckDuckGo,
    Wikipedia,
}

pub struct SearchSuggestionsPref {
    pub ahead_of_browsing_history: Option<bool>,
    pub in_private_windows: Option<bool>,
    pub recent_searches: Option<bool>,
}

pub struct AddressBarPref {
    pub browsing_history: Option<bool>,
    pub bookmarks: Option<bool>,
    pub clipboard: Option<bool>,
    pub open_tabs: Option<bool>,
    pub shortcuts: Option<bool>,
    pub search_engines: Option<bool>,
}

pub struct PrivacyAndSecurityPref {
    pub tracking_protection: Option<TrackingProtectionPref>,
    pub tell_sites_not_to_sell_or_share_data: Option<bool>,
    pub delete_cookies_when_closed: Option<bool>,
    pub passwords: Option<PasswordsPref>,
    pub autofill: Option<AutofillPref>,
    pub remember_history: Option<RememberHistoryPref>,
    pub block_popups: Option<bool>,
    pub warn_about_addon_installs: Option<bool>,
    pub security: Option<SecurityPref>,
    pub dns_over_https: Option<DnsOverHttpsPref>,
}

pub struct SecurityPref {
    pub block_dangerous_and_deceptive_content: Option<bool>,
    pub block_dangerous_downloads: Option<bool>,
    pub warn_about_unwanted_uncommon_software: Option<bool>,
    pub query_ocsp_responder_to_confirm_certificates: Option<bool>,
    pub auto_trust_third_party_root_certificates: Option<bool>,
    pub https_only_mode: Option<HttpsOnlyModePref>,
}

pub enum HttpsOnlyModePref {
    InAllWindows,
    InPrivateWindowsOnly,
    DontEnable,
}

pub struct DnsOverHttpsPref {
    pub mode: Option<DnsOverHttpsModePref>,
}

pub enum DnsOverHttpsModePref {
    DefaultProtection,
    IncreasedProtection,
    MaxProtection,
    Off,
}

pub enum TrackingProtectionPref {
    Standard,
    Strict(StrictTrackingProtectionPref),
    //Custom,
}

pub struct StrictTrackingProtectionPref {
    pub allow_browser_to_apply_exceptions: Option<bool>,
    pub allow_browser_to_apply_minor_exceptions: Option<bool>,
}

pub struct PasswordsPref {
    pub ask_to_save_passwords: Option<AskToSavePasswordsPref>,
    pub require_device_signin_to_fill: Option<bool>,
    pub primary_password: Option<PrimaryPasswordPref>,
    pub windows_sso: Option<bool>,
}

pub struct AskToSavePasswordsPref {
    pub fill_usernames_and_passwords: Option<bool>,
    pub suggest_strong_passwords: Option<bool>,
    pub suggest_firefox_relay_email: Option<bool>,
    pub alerts_about_passwords_breached_websites: Option<bool>,
}

pub struct PrimaryPasswordPref {
    pub enabled: Option<bool>,
    pub password: Option<&'static str>,
}

pub struct AutofillPref {
    pub save_and_fill_payment_methods: Option<bool>,
    pub require_device_signin_to_fill: Option<bool>,
}

pub enum RememberHistoryPref {
    RememberHistory,
    NeverRemember,
    CustomSettings(CustomRememberHistorySettings),
}

pub struct CustomRememberHistorySettings {
    pub browsing_and_download_history: Option<bool>,
    pub search_and_form_history: Option<bool>,
    pub clear_history_when_closing: Option<bool>,
}

impl Funcs for ZenBrowser {
    fn id(&self) -> &'static str {
        "Zen Browser"
    }

    fn enable(&self) -> Result<()> {
        log::debug!("Enabling 'Zen Browser' module...");

        // Ensure parent dirs exist (util functions already add context)
        util::ensure_parent_dir(paths::POLICY_PATH)?;
        util::ensure_parent_dir(paths::AUTOCONFIG_JS)?;
        util::ensure_parent_dir(paths::MOZILLA_CFG)?;

        // 1) policies.json
        let policies_value = policies_to_json(&self.policies);
        let policies_pretty =
            serde_json::to_string_pretty(&policies_value).context("serialize policies JSON")?;
        fs::write(paths::POLICY_PATH, policies_pretty)
            .with_context(|| format!("write '{}'", paths::POLICY_PATH))?;
        log::debug!("'policies.json' written to '{}'", paths::POLICY_PATH);

        // 2) autoconfig.js
        let autoconfig = util::render_autoconfig_js();
        fs::write(paths::AUTOCONFIG_JS, autoconfig)
            .with_context(|| format!("write '{}'", paths::AUTOCONFIG_JS))?;
        log::debug!("'autoconfig.js' written to '{}'", paths::AUTOCONFIG_JS);

        // 3) mozilla.cfg
        let cfg = render_cfg_from_prefs(&self.prefs);
        fs::write(paths::MOZILLA_CFG, cfg)
            .with_context(|| format!("write '{}'", paths::MOZILLA_CFG))?;
        log::debug!("'mozilla.cfg' written to '{}'", paths::MOZILLA_CFG);

        Ok(())
    }

    fn disable(&self) -> Result<()> {
        util::remove_file_silent(paths::POLICY_PATH)?;
        log::debug!("'policies.json' removed from '{}'", paths::POLICY_PATH);

        util::remove_file_silent(paths::AUTOCONFIG_JS)?;
        log::debug!("'autoconfig.js' removed from '{}'", paths::AUTOCONFIG_JS);

        util::remove_file_silent(paths::MOZILLA_CFG)?;
        log::debug!("'mozilla.cfg' removed from '{}'", paths::MOZILLA_CFG);

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

    if let Some(_) = p.restore_prev_session {
        out.push(("browser.startup.page", Int(3)));
    }
    if let Some(_) = p.continue_where_you_left_off {
        out.push(("zen.workspaces.continue-where-left-off", Bool(true)));
    }

    out.extend_from_slice(&p.extra);

    util::render_mozilla_cfg(&out)
}
