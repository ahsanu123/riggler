import m, { render } from 'mithril'
import './styles.css'

const mainContainer = document.getElementById("main-container")

enum ActiveTab {
  MainScreen,
  Setting,
  About
}

interface RigglerAppState {
  activeTab: ActiveTab,
  jigglingDuration: number,
  jigglingDelta: number,
}

const rigglerAppState: RigglerAppState = {
  activeTab: ActiveTab.MainScreen,
  jigglingDuration: 1,
  jigglingDelta: 1
}

const mainScreenTab: m.Component = {
  view: function (): m.Children | null | void {
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
                name: "is-jiggling-checkbox"
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
                width: "50px",
                src: "/src/assets/chevron-down-icon.svg"
              })
            ),

            m('button', {
              onclick: () => rigglerAppState.activeTab = ActiveTab.Setting
            },
              m("img", {
                width: "50px",
                src: "/src/assets/gear-icon.svg"
              })
            ),

            m('button', {
              onclick: () => rigglerAppState.activeTab = ActiveTab.About
            },
              m("img", {
                width: "50px",
                src: "/src/assets/question-mark.svg"
              })
            ),
          ]
        )
      ]
    )
  }
}

const aboutScreenTab: m.Component = {
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
          `
            Die Studierenden sollen die Zusammenhänge zwischen den verschiedenen Größen der
            Feldtheorie verstehen und in mathematischer
            Form anwenden können. Darüber hinaus
            sollen sie die Vorgänge in stationären, qu
          `)
      ])
    ])
  }
}

const settingScreenTab: m.Component = {
  view: function (): m.Children | null | void {
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
            value: rigglerAppState.jigglingDuration
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
            value: rigglerAppState.jigglingDuration
          }),
        ]
      )
    ])
  }
}

const rigglerApp: m.Component = {

  view: function (): m.Children | null | void {

    const renderActiveTab = () => {

      switch (rigglerAppState.activeTab) {
        case ActiveTab.MainScreen:
          return m(mainScreenTab)

        case ActiveTab.Setting:
          return m(settingScreenTab)

        case ActiveTab.About:
          return m(aboutScreenTab)

        default:
          return m(mainScreenTab)
      }
    }

    return m('.riggler-app', renderActiveTab())
  }
}

m.mount(mainContainer!, rigglerApp)

