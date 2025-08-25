use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub mod config;
pub mod logger;
pub mod modules;

pub type CheckFn = Box<dyn FnMut(&State) -> bool>;
pub type EnableFn = Box<dyn FnMut(&mut State)>;
pub type DisableFn = Box<dyn FnMut(&mut State)>;

#[derive(Clone, Copy)]
pub struct State {
    pub enabled: bool,
}

pub struct Module {
    pub state: State,
    pub check_fn: CheckFn,
    pub enable_fn: EnableFn,
    pub disable_fn: DisableFn,
    pub submodules: Vec<Module>,
}

impl Module {
    pub fn check(&mut self) -> bool {
        (self.check_fn)(&self.state)
    }

    pub fn enable(&mut self) {
        (self.enable_fn)(&mut self.state);
    }

    pub fn disable(&mut self) {
        (self.disable_fn)(&mut self.state);
    }
}

pub struct ModuleBuilder {
    pub state: State,
    pub check_fn: CheckFn,
    pub enable_fn: EnableFn,
    pub disable_fn: DisableFn,
    submodules: Vec<Module>,
}

impl ModuleBuilder {
    pub fn new() -> ModuleBuilder {
        ModuleBuilder {
            state: State { enabled: false },
            check_fn: Box::new(|s: &State| s.enabled),
            enable_fn: Box::new(|s: &mut State| s.enabled = true),
            disable_fn: Box::new(|s: &mut State| s.enabled = false),
            submodules: Vec::new(),
        }
    }

    pub fn set_check_fn(mut self, f: impl FnMut(&State) -> bool + 'static) -> Self {
        self.check_fn = Box::new(f);
        self
    }

    pub fn set_enable_fn(mut self, f: impl FnMut(&mut State) + 'static) -> Self {
        self.enable_fn = Box::new(f);
        self
    }

    pub fn set_disable_fn(mut self, f: impl FnMut(&mut State) + 'static) -> Self {
        self.disable_fn = Box::new(f);
        self
    }

    pub fn add_submodule(mut self, module: Module) -> Self {
        self.submodules.push(module);
        self
    }

    pub fn build(self) -> Module {
        Module {
            state: self.state,
            check_fn: self.check_fn,
            enable_fn: self.enable_fn,
            disable_fn: self.disable_fn,
            submodules: self.submodules,
        }
    }
}

pub struct SystemManager<T: Eq + Hash + Debug> {
    modules: HashMap<T, Module>,
}

impl<T: Eq + Hash + Debug> SystemManager<T> {
    pub fn new() -> Self {
        SystemManager {
            modules: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, id: T, module: Module) {
        self.modules.insert(id, module);
    }

    pub fn apply(&mut self) {
        log::info!("Applying system configuration...");
        for (id, module) in &mut self.modules {
            log::debug!("Applying module: '{:?}'", id);
            SystemManager::<T>::apply_recursive(module);
        }
        log::info!("Configuration apply complete.");
    }

    fn apply_recursive(module: &mut Module) {
        if !module.check() {
            module.enable();
        }
        if module.check() {
            for sub_module in module.submodules.iter_mut() {
                SystemManager::<T>::apply_recursive(sub_module);
            }
        }
    }
}
