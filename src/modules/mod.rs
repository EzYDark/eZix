pub mod control_panel;
pub mod firewall;
pub mod zen_browser;

use crate::{Funcs, modules::zen_browser::ZenBrowser};
use control_panel::ControlPanel;
use firewall::Firewall;

pub fn all_modules() -> Vec<Box<dyn Funcs>> {
    vec![
        Box::new(Firewall::default()),
        Box::new(ControlPanel::default()),
        Box::new(ZenBrowser::default()),
    ]
}
