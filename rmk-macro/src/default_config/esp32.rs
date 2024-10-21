use rmk_config::toml_config::{BleConfig, StorageConfig};

use crate::{
    keyboard_config::{CommunicationConfig, KeyboardConfig},
    ChipModel,
};

// Default config for esp32
pub(crate) fn default_esp32(chip: ChipModel) -> KeyboardConfig {
    KeyboardConfig {
        chip,
        communication: CommunicationConfig::Ble(BleConfig {
            enabled: true,
            ..Default::default()
        }),
        storage: StorageConfig {
            start_addr: Some(0),
            num_sectors: Some(32),
            enabled: true,
        },
        ..Default::default()
    }
}
