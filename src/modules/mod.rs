pub mod control_panel;
pub mod firewall;
pub mod xserver;

use crate::Funcs;
use control_panel::ControlPanel;
use firewall::Firewall;
use xserver::XServer;

pub fn all_modules() -> Vec<Box<dyn Funcs>> {
    vec![
        Box::new(Firewall::default()),
        Box::new(XServer::default()),
        Box::new(ControlPanel::default()),
    ]
}
