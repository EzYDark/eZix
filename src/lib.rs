pub mod modules;
use std::collections::HashMap;

pub trait Funcs {
    fn name(&self) -> &'static str;
    fn enable(&self);
    fn disable(&self);
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

    pub fn combine(mut self, other: SystemConfig<'a>) -> Self {
        self.modules.extend(other.modules);
        self
    }

    pub fn apply(&self) {
        let all_modules = modules::all_modules();
        let declared_modules: HashMap<&str, &Box<dyn Funcs + 'a>> =
            self.modules.iter().map(|m| (m.name(), m)).collect();

        for module in all_modules {
            if let Some(declared_module) = declared_modules.get(module.name()) {
                if declared_module.is_enabled() {
                    declared_module.enable();
                } else {
                    declared_module.disable();
                }
            } else {
                module.disable();
            }
        }
    }
}

#[macro_export]
macro_rules! ezix {
    ($($module:expr),*) => {
        $crate::SystemConfig::new()
            $(.with($module))*
    };
}
