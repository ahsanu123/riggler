use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct RigglerConfig {
    pub jiggling_delta: i32,
    pub jiggling_delay: i32,
}

impl Default for RigglerConfig {
    fn default() -> Self {
        Self {
            jiggling_delta: 1,
            jiggling_delay: 1,
        }
    }
}
