import { getConfig, setConfig, toggleJiggling } from "./invokes"

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

  trayButton = document.getElementById("minimize-to-tray-button") as HTMLButtonElement
  settingButton = document.getElementById("setting-button") as HTMLButtonElement

  jigglingCheckbox = document.getElementById("jiggling-checkbox") as HTMLInputElement
  jigglingDuration = document.getElementById("jiggling-duration") as HTMLInputElement
  jigglingDurationLabel = document.getElementById("jiggling-duration-label") as HTMLLabelElement

  settingContainer = document.getElementById("riggler-setting") as HTMLDivElement
  helpContainer = document.getElementById("riggler-about") as HTMLDivElement

  getConfig().then(value => {
    const duration = value?.jiggling_duration ?? 1
    state.jigglingDuration = duration
    jigglingDurationLabel!.innerText = `Jiggling Duration: ${duration}`
    jigglingDuration!.value = duration.toString()
  })

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

  const handleOnJigglingCheckedChange = async (_ev: Event) => {
    const result = await toggleJiggling()
    console.log("toggleJiggling", result)
  }

  const handleOnJigglingDurationChange = async (ev: Event) => {
    const range = ev.target as HTMLInputElement
    jigglingDurationLabel!.innerText = `Jiggling Duration: ${range.value}`

    state.jigglingDuration = Number(range.value)
    const result = await setConfig({ jiggling_duration: state.jigglingDuration })

    console.log("setconfig", result)
  }

  trayButton.addEventListener('click', handleOnTrayButtonClick)
  settingButton.addEventListener('click', toggleSettingContainerVisibility)

  jigglingCheckbox.addEventListener('change', handleOnJigglingCheckedChange)
  jigglingDuration.addEventListener('change', handleOnJigglingDurationChange)

});
