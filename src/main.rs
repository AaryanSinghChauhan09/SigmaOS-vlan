use ::vlan::{vlan, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = vlan::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.start().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.stop().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-vlan operational");
    Ok(())
}
