import { getCurrentWindow, LogicalPosition, LogicalSize, PhysicalSize } from "@tauri-apps/api/window";

let trayButton: HTMLButtonElement | null
let settingButton: HTMLButtonElement | null

let jigglingCheckbox: HTMLInputElement | null
let jigglingDuration: HTMLInputElement | null
let jigglingDurationLabel: HTMLLabelElement | null

let settingContainer: HTMLDivElement | null
let helpContainer: HTMLDivElement | null

let state = {
  isSettingHidden: true,
  isJiggling: false,
  jigglingDuration: 1
}

window.addEventListener("DOMContentLoaded", () => {

  // const _appWindow = getCurrentWindow()

  trayButton = document.getElementById("minimize-to-tray-button") as HTMLButtonElement
  settingButton = document.getElementById("setting-button") as HTMLButtonElement

  jigglingCheckbox = document.getElementById("jiggling-checkbox") as HTMLInputElement
  jigglingDuration = document.getElementById("jiggling-duration") as HTMLInputElement
  jigglingDurationLabel = document.getElementById("jiggling-duration-label") as HTMLLabelElement

  settingContainer = document.getElementById("riggler-setting") as HTMLDivElement
  helpContainer = document.getElementById("riggler-about") as HTMLDivElement

  const handleOnTrayButtonClick = () => {
    // TODO: invoke tauri or something
    // need reading more about system tray
  }

  const toggleSettingContainerVisibility = () => {
    if (state.isSettingHidden) {
      settingContainer?.removeAttribute('hidden')
      state.isSettingHidden = false

      helpContainer?.setAttribute('hidden', '')
    }
    else {
      settingContainer?.setAttribute('hidden', '')
      state.isSettingHidden = true

      helpContainer?.removeAttribute('hidden')
    }
  }

  const handleOnJigglingCheckedChange = (ev: Event) => {
    console.log("onJiggling", ev)
  }

  const handleOnJigglingDurationChane = (ev: Event) => {
    const range = ev.target as HTMLInputElement
    jigglingDurationLabel!.innerText = `Jiggling Duration: ${range.value}`
    state.jigglingDuration = Number(range.value)
    console.log("onJigglingDurationChange", ev, state.jigglingDuration)
  }

  trayButton.addEventListener('click', handleOnTrayButtonClick)
  settingButton.addEventListener('click', toggleSettingContainerVisibility)

  jigglingCheckbox.addEventListener('change', handleOnJigglingCheckedChange)
  jigglingDuration.addEventListener('change', handleOnJigglingDurationChane)

});
