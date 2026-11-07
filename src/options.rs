//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Show,
    Add,
    Delete,
    Set,
}

#[derive(Debug, Clone)]
pub struct VlanOptions {
    pub mode: Mode,
    pub interface: Option<String>,
    pub vlan_id: Option<u16>,
}

impl Default for VlanOptions {
    fn default() -> Self { Self { mode: Mode::Default, interface: None, vlan_id: None } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(VlanOptions::default().mode, Mode::Default); }
}
