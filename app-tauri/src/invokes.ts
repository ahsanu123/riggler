import { invoke } from "@tauri-apps/api/core"
import { RigglerConfig } from "./riggler-config"

const TOGGLE_JIGGLING_INVOKE_NAME = "toggle_jiggling"
const GET_CONFIG_INVOKE_NAME = "get_config"
const SET_CONFIG_INVOKE_NAME = "set_config"

export async function toggleJigglingInvoke(): Promise<boolean> {
  try {
    const result = await invoke<boolean>(TOGGLE_JIGGLING_INVOKE_NAME)
    return result
  } catch (error) {
    // TODO: add notification here
    return false
  }
}

export async function getConfigInvoke(): Promise<RigglerConfig | undefined> {
  try {
    const result = await invoke<RigglerConfig>(GET_CONFIG_INVOKE_NAME)
    return result
  } catch (error) {
    // TODO: add notification here
  }
}

export async function setConfigInvoke(config: RigglerConfig): Promise<boolean> {
  try {
    const result = await invoke<boolean>(SET_CONFIG_INVOKE_NAME, { config })
    return result
  } catch (error) {
    // TODO: add notification here
    return false
  }
}
