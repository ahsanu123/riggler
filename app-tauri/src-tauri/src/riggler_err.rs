use tauri::ipc::InvokeError;

#[derive(Debug)]
pub enum RigglerErr {
    GetStoreProviderErr,
    SetConfigErr,
    GetConfigErr,
    SaveStoreErr,
}

impl Into<InvokeError> for RigglerErr {
    fn into(self) -> InvokeError {
        match self {
            RigglerErr::GetStoreProviderErr => InvokeError::from("error when get store provider"),
            RigglerErr::SetConfigErr => InvokeError::from("error when setting config"),
            RigglerErr::GetConfigErr => InvokeError::from("error when get config"),
            RigglerErr::SaveStoreErr => InvokeError::from("error save config"),
        }
    }
}
