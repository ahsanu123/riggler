use riggler_shared::JIGGLING_ENABLE;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::sync::atomic::Ordering;
use tauri::{ipc::InvokeError, AppHandle};
use tauri_plugin_store::StoreExt;

const RIGGLER_SETTING: &str = "riggler.setting.json";
const RIGGLER_SETTING_KEY: &str = "riggler_setting";

#[derive(Debug)]
pub enum RigglerErr {
    GetStoreProviderErr,
    SetConfigErr,
    GetConfigErr,
    ToggleJigglingErr,
    SaveStoreErr,
}

impl Into<InvokeError> for RigglerErr {
    fn into(self) -> InvokeError {
        match self {
            RigglerErr::GetStoreProviderErr => InvokeError::from("error when get store provider"),
            RigglerErr::SetConfigErr => InvokeError::from("error when setting config"),
            RigglerErr::GetConfigErr => InvokeError::from("error when get config"),
            RigglerErr::ToggleJigglingErr => InvokeError::from("error when toggle jiggling"),
            RigglerErr::SaveStoreErr => InvokeError::from("error save config"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RigglerConfig {
    jiggling_duration: u32,
}

impl RigglerConfig {
    pub fn new(jiggling_duration: u32) -> Self {
        Self { jiggling_duration }
    }
}

impl Default for RigglerConfig {
    fn default() -> Self {
        Self {
            jiggling_duration: 1,
        }
    }
}

#[tauri::command]
pub fn toggle_jiggling() -> bool {
    let jiggling = JIGGLING_ENABLE.load(Ordering::Relaxed);

    JIGGLING_ENABLE.store(!jiggling, Ordering::Relaxed);

    !jiggling
}

#[tauri::command]
pub fn get_config(app: AppHandle) -> Result<RigglerConfig, RigglerErr> {
    let store = app
        .store(RIGGLER_SETTING)
        .map_err(|_| RigglerErr::GetStoreProviderErr)?;

    let riggler_config_value = store.get(RIGGLER_SETTING_KEY);

    if riggler_config_value.is_none() {
        return Ok(RigglerConfig::default());
    } else {
        let config = serde_json::from_value::<RigglerConfig>(riggler_config_value.unwrap())
            .map_err(|_| RigglerErr::GetConfigErr)?;
        return Ok(config);
    }
}

#[tauri::command]
pub fn set_config(app: AppHandle, config: RigglerConfig) -> Result<bool, RigglerErr> {
    let store = app
        .store(RIGGLER_SETTING)
        .map_err(|_| RigglerErr::GetStoreProviderErr)?;

    let config_value = serde_json::to_value(config).map_err(|_| RigglerErr::SetConfigErr)?;

    store.set(RIGGLER_SETTING_KEY, config_value);
    store.save().map_err(|_| RigglerErr::SaveStoreErr)?;

    Ok(true)
}
