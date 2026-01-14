import m from 'mithril'
import { ActiveTab, rigglerAppState } from './state'

export const mainScreen: m.Component = {
  view: function (): m.Children | null | void {
    const handleOnIsJigglingCheckedChange = (checked: boolean) => {
      rigglerAppState.isJiggling = checked
    }
    return m(".main-screen",
      [
        m('img', {
          onclick: () => rigglerAppState.activeTab = ActiveTab.MainScreen,
          class: 'riggler-icon-image',
          src: '/src/assets/riggler-icon.png',
          height: '70px',
          style: {
            borderRadius: "60px"
          }
        }),

        m('.jiggling-container',
          [
            m("h2", "Riggler"),
            m("div", [
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
            ])
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

