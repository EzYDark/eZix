use crate::{Module, ModuleBuilder, State};

#[derive(Clone, Default)]
pub struct XServerConfig {
    pub enable: bool,
    pub layout: &'static str,
    pub gnome_enable: bool,
}

pub fn build(config: XServerConfig) -> Module {
    let mut builder = ModuleBuilder::new().set_enable_fn(move |s: &mut State| {
        log::info!("Enabling XServer with layout: '{}'", config.layout);
        s.enabled = true;
    });

    if config.gnome_enable {
        let gnome_submodule = ModuleBuilder::new()
            .set_enable_fn(|_| {
                log::info!("-> Enabling Gnome Desktop...");
            })
            .build();
        builder = builder.add_submodule(gnome_submodule);
    }

    builder.build()
}
