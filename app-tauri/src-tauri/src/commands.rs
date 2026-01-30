use crate::{riggler_config::RigglerConfig, riggler_err::RigglerErr};
use riggler_shared::{JIGGLING_DELAY, JIGGLING_DELTA, JIGGLING_ENABLE};
use std::sync::atomic::Ordering;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const RIGGLER_SETTING: &str = "riggler.setting.json";
const RIGGLER_SETTING_KEY: &str = "riggler_setting";

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
        Ok(RigglerConfig::default())
    } else {
        let config = serde_json::from_value::<RigglerConfig>(riggler_config_value.unwrap())
            .map_err(|_| RigglerErr::GetConfigErr)?;

        if config.jiggling_delta != JIGGLING_DELTA.load(Ordering::Relaxed) {
            JIGGLING_DELTA.store(config.jiggling_delta, Ordering::Relaxed);
        }

        if config.jiggling_delay != JIGGLING_DELAY.load(Ordering::Relaxed) {
            JIGGLING_DELAY.store(config.jiggling_delay, Ordering::Relaxed);
        }
        Ok(config)
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

    JIGGLING_DELAY.store(config.jiggling_delay, Ordering::Relaxed);
    JIGGLING_DELTA.store(config.jiggling_delta, Ordering::Relaxed);

    Ok(true)
}
