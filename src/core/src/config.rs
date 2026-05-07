use crate::cli::CfgOptions;
use anyhow::{Result, anyhow};
use std::sync::OnceLock;

static INSTANCE: OnceLock<ZynxConfigs> = OnceLock::new();

#[derive(Debug)]
pub struct ZynxConfigs {
    pub enable_debugger: bool,
    pub enable_zygisk: bool,
    pub enable_liteloader: bool,
}

impl ZynxConfigs {
    pub fn init(config: &CfgOptions) -> Result<()> {
        let instance = Self {
            enable_debugger: config.cfg_enable_debugger,
            enable_zygisk: config.cfg_enable_zygisk,
            enable_liteloader: config.cfg_enable_liteloader,
        };

        INSTANCE
            .set(instance)
            .map_err(|_| anyhow!("duplicate called"))?;

        Ok(())
    }

    pub fn instance() -> &'static Self {
        INSTANCE.get().expect("configs not initialized")
    }
}
