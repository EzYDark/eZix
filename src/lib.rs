pub mod modules;
pub mod tools;
use anyhow::{Result, anyhow};
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
        SystemConfig {
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
        let declared_modules: HashMap<&str, &Box<dyn Funcs + 'a>> =
            self.modules.iter().map(|m| (m.id(), m)).collect();

        for module in all_modules {
            if let Some(declared_module) = declared_modules.get(module.id()) {
                if declared_module.is_enabled() {
                    match declared_module.enable() {
                        Ok(_) => log::debug!("'{}' module enabled successfully", module.id()),
                        Err(e) => {
                            return Err(anyhow!(
                                "Failed to enable '{}' module:\n - {}",
                                module.id(),
                                e
                            ));
                        }
                    }
                } else {
                    match declared_module.disable() {
                        Ok(_) => log::debug!("'{}' module disabled successfully", module.id()),
                        Err(e) => {
                            return Err(anyhow!(
                                "Failed to disable '{}' module:\n - {}",
                                module.id(),
                                e
                            ));
                        }
                    };
                }
            } else {
                match module.disable() {
                    Ok(_) => log::debug!("'{}' module disabled successfully", module.id()),
                    Err(e) => {
                        return Err(anyhow!(
                            "Failed to disable '{}' module:\n - {}",
                            module.id(),
                            e
                        ));
                    }
                };
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! ezix {
    ($($module:expr),*) => {
        $crate::SystemConfig::new()
            $(.with($module))*
    };
}
