import m from 'mithril'
import { ActiveTab, rigglerAppState } from './state'

export const settingScreen: m.Component = {
  view: function (): m.Children | null | void {
    const handleOnJigglingDurationChange = (duration: number) => {
      // TODO: add invoke to change duration in rust
      rigglerAppState.jigglingDuration = duration
    }
    const handleOnJigglingDeltaChange = (value: number) => {
      // TODO: add invoke to change value in rust
      rigglerAppState.jigglingDelta = value
    }

    return m(".setting-screen", [
      m('img', {
        onclick: () => rigglerAppState.activeTab = ActiveTab.MainScreen,
        class: 'riggler-icon-image',
        src: '/src/assets/riggler-icon.png',
        height: '70px',
        style: {
          borderRadius: "60px"
        }
      }),

      m("div",
        [
          m("label", { for: "jiggling-duration" }, `Jiggling Duration ${rigglerAppState.jigglingDuration}`),
          m("br"),
          m("input[type='range']", {
            id: "jiggling-duration",
            name: "jiggling-duration",
            class: "jiggling-duration",
            min: 1,
            max: 10,
            value: rigglerAppState.jigglingDuration,
            oninput: (e: Event) => {
              const value = (e.target as HTMLInputElement).value
              handleOnJigglingDurationChange(Number(value))
            }
          }),

          m("br"),
          m("label", { for: "jiggling-delta" }, `Jiggling Delta ${rigglerAppState.jigglingDelta}`),
          m("br"),
          m("input[type='range']", {
            id: "jiggling-delta",
            name: "jiggling-delta",
            class: "jiggling-delta",
            min: 1,
            max: 10,
            value: rigglerAppState.jigglingDelta,
            oninput: (e: Event) => {
              const value = (e.target as HTMLInputElement).value
              handleOnJigglingDeltaChange(Number(value))
            }
          }),
        ]
      )
    ])
  }
}
