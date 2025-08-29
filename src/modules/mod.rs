pub mod control_panel;
pub mod firewall;

use crate::Funcs;
use control_panel::ControlPanel;
use firewall::Firewall;

pub fn all_modules() -> Vec<Box<dyn Funcs>> {
    vec![
        Box::new(Firewall::default()),
        Box::new(ControlPanel::default()),
    ]
}
