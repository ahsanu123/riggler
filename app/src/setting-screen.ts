import m from 'mithril'
import { ActiveTab, rigglerAppState } from './state'
import { getConfigInvoke, setConfigInvoke } from './invokes'

const MAX_DELAY = 5;
const MAX_DELTA = 25;
export const settingScreen: m.Component = {
  view: function (): m.Children | null | void {
    const handleOnJigglingDurationChange = async (duration: number) => {
      rigglerAppState.jigglingDuration = duration

      let config = await getConfigInvoke();
      if (config) {
        config.jiggling_delay = duration
        setConfigInvoke(config)
      }
    }
    const handleOnJigglingDeltaChange = async (value: number) => {
      rigglerAppState.jigglingDelta = value

      let config = await getConfigInvoke();
      if (config) {
        config.jiggling_delta = value
        setConfigInvoke(config)
      }
    }

    return m(".setting-screen", [
      m('img', {
        onclick: () => rigglerAppState.activeTab = ActiveTab.MainScreen,
        class: 'riggler-icon-image',
        src: '/src/assets/riggler-icon1.png',
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
            max: MAX_DELAY,
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
            max: MAX_DELTA,
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
