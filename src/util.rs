//! Vlan model — manage VLAN interfaces.

#[derive(Debug, Clone)]
pub struct VlanInfo {
    pub iface: String,
    pub vlan_id: u16,
    pub parent: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct Vlan {
    pub show_details: bool,
}

impl Default for Vlan {
    fn default() -> Self { Self { show_details: false } }
}

impl Vlan {
    pub fn new() -> Self { Self::default() }

    pub fn list(&self) -> Vec<VlanInfo> {
        vec![VlanInfo { iface: "eth0.100".into(), vlan_id: 100, parent: "eth0".into(), state: "UP".into() }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn list() {
        let v = Vlan::new();
        assert_eq!(v.list()[0].vlan_id, 100);
    }
}
