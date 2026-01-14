import m from 'mithril'
import { ActiveTab, rigglerAppState } from './state'
import { settingScreen } from './setting-screen'
import { aboutScreen } from './about-screen'
import { mainScreen } from './main-screen'
import './styles.css'

const mainContainer = document.getElementById("main-container")

const rigglerApp: m.Component = {

  view: function (): m.Children | null | void {

    const renderActiveTab = () => {

      switch (rigglerAppState.activeTab) {
        case ActiveTab.MainScreen:
          return m(mainScreen)

        case ActiveTab.Setting:
          return m(settingScreen)

        case ActiveTab.About:
          return m(aboutScreen)

        default:
          return m(mainScreen)
      }
    }

    return m('.riggler-app', renderActiveTab())
  }
}

m.mount(mainContainer!, rigglerApp)

