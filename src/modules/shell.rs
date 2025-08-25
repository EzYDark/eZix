use crate::{Module, ModuleBuilder, State};

#[derive(Clone, Debug, Default)]
pub enum PackageManager {
    #[default]
    Winget,
    Scoop,
    Chocolatey,
}

#[derive(Clone, Debug, Default)]
pub struct ShellConfig {
    pub enable: bool,
    pub package_manager: PackageManager,
    pub packages: Vec<&'static str>,
}

pub fn build(config: ShellConfig) -> Module {
    ModuleBuilder::new()
        .set_enable_fn(move |s: &mut State| {
            log::info!("Configuring user shell via {:?}", config.package_manager);
            log::info!("Ensuring packages are installed: {:?}", config.packages);
            // Real implementation would run scoop/winget install commands.
            s.enabled = true;
        })
        .build()
}
