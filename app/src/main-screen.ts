import m from 'mithril'
import { ActiveTab, rigglerAppState } from './state'
import { toggleJigglingInvoke } from './invokes'

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
          src: '/src/assets/riggler-icon1.png',
          height: '70px',
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
                src: "/src/assets/chevron-down-icon.svg"
              })
            ),

            m('button', {
              onclick: () => rigglerAppState.activeTab = ActiveTab.Setting
            },
              m("img", {
                width: "30px",
                src: "/src/assets/gear-icon.svg"
              })
            ),

            m('button', {
              onclick: () => rigglerAppState.activeTab = ActiveTab.About
            },
              m("img", {
                width: "30px",
                src: "/src/assets/question-mark.svg"
              })
            ),
          ]
        )
      ]
    )
  }
}

