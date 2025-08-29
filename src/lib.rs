pub mod modules;
use anyhow::{Context, Result};
use std::collections::HashMap;

pub trait Funcs {
    fn id(&self) -> &'static str;
    fn enable(&self) -> Result<()>;
    fn disable(&self) -> Result<()>;
    fn is_enabled(&self) -> bool;
}

pub struct SystemConfig<'a> {
    pub modules: Vec<Box<dyn Funcs + 'a>>,
}

impl<'a> SystemConfig<'a> {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    pub fn with(mut self, module: impl Funcs + 'a) -> Self {
        self.modules.push(Box::new(module));
        self
    }

    pub fn extend(mut self, other: SystemConfig<'a>) -> Self {
        self.modules.extend(other.modules);
        self
    }

    pub fn apply(&self) -> Result<()> {
        let all_modules = modules::all_modules();
        let declared: HashMap<&str, &Box<dyn Funcs + 'a>> =
            self.modules.iter().map(|m| (m.id(), m)).collect();

        for default_mod in all_modules {
            let id = default_mod.id();

            if let Some(cfg_mod) = declared.get(id) {
                if cfg_mod.is_enabled() {
                    log::info!("Enabling module '{}'", id);
                    cfg_mod
                        .enable()
                        .with_context(|| format!("enable '{}'", id))?;
                    log::debug!("'{}' module enabled", id);
                } else {
                    log::info!("Disabling module '{}'", id);
                    cfg_mod
                        .disable()
                        .with_context(|| format!("disable '{}'", id))?;
                    log::debug!("'{}' module disabled", id);
                }
            } else {
                log::info!("Disabling undeclared module '{}'", id);
                default_mod
                    .disable()
                    .with_context(|| format!("disable undeclared '{}'", id))?;
                log::debug!("'{}' module disabled (undeclared)", id);
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! ezix {
    ($($module:expr),* $(,)?) => {
        $crate::SystemConfig::new()
            $(.with($module))*
    };
}
