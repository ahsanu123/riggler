import m from 'mithril'
import { ActiveTab, rigglerAppState } from './state'

export const aboutScreen: m.Component = {
  view: function (): m.Children | null | void {
    return m(".about-screen", [
      m('img', {
        onclick: () => rigglerAppState.activeTab = ActiveTab.MainScreen,
        class: 'riggler-icon-image',
        src: '/src/assets/riggler-icon.png',
        height: '70px',
        style: {
          borderRadius: "60px"
        }
      }),

      m("div", [
        m("h2", "About Riggler"),
        m("p", {
          style: { padding: '0 10px 0 0' }
        },
          "Dead Simple Mouse Jiggler Implemented in Rust."
        ),
        m("sub", {
          style: { padding: '0 10px 0 0' }
        },
          "github.com/ahsanu123/riggler"
        )
      ])
    ])
  }
}

