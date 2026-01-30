import m from 'mithril'
import { ActiveTab, rigglerAppState } from './state'
import { toggleJigglingInvoke } from './invokes'
import logo from './assets/riggler-icon1.png'
import chevronDown from './assets/chevron-down-icon.svg'
import gear from './assets/gear-icon.svg'
import questionMark from './assets/question-mark.svg'

export const mainScreen: m.Component = {
  view: function (): m.Children | null | void {
    const handleOnIsJigglingCheckedChange = async (checked: boolean) => {
      rigglerAppState.isJiggling = checked
      const isJiggling = await toggleJigglingInvoke()

      if (checked !== isJiggling) await toggleJigglingInvoke()
    }
    return m(".main-screen",
      [
        m('img', {
          onclick: () => rigglerAppState.activeTab = ActiveTab.MainScreen,
          class: 'riggler-icon-image',
          src: logo,
          height: '70px',
          draggable: 'false',
          style: {
            borderRadius: "60px"
          }
        }),

        m('.jiggling-container',
          [
            m("input[type='checkbox']", {
              id: "is-jiggling-checkbox",
              name: "is-jiggling-checkbox",
              checked: rigglerAppState.isJiggling,
              onclick: (e: Event) => {
                const checked = (e.target as HTMLInputElement).checked
                handleOnIsJigglingCheckedChange(checked)
              }
            }),
            m("label",
              { for: "is-jiggling-checkbox" },
              "Jiggling"
            )
          ]
        ),

        m(".button-container",
          [
            m('button', {
              onclick: () => console.log("todo minimize to tray")
            },
              m("img", {
                width: "30px",
                draggable: 'false',
                src: chevronDown
              })
            ),

            m('button', {
              onclick: () => rigglerAppState.activeTab = ActiveTab.Setting
            },
              m("img", {
                width: "30px",
                draggable: 'false',
                src: gear
              })
            ),

            m('button', {
              onclick: () => rigglerAppState.activeTab = ActiveTab.About
            },
              m("img", {
                width: "30px",
                draggable: 'false',
                src: questionMark
              })
            ),
          ]
        )
      ]
    )
  }
}

